use crate::ast::*;
use crate::lexer::Token;

pub struct Parser<'a> {
    tokens: std::iter::Peekable<std::slice::Iter<'a, Token>>,
    _source: &'a str,
    pub errors: Vec<String>,
    pub ast: Vec<ASTNode>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
            _source: source,
            errors: Vec::new(),
            ast: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> &Vec<ASTNode> {
        loop {
            let token = match self.peek() {
                Some(t) => t.clone(),
                None => break,
            };
            match token {
                Token::Eof => {
                    self.next();
                    break;
                }
                Token::Function => {
                    self.next();
                    if let Some(stmt) = self.parse_function_decl(false) {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Local => {
                    self.next();
                    if self.check_next(Token::Function) {
                        if let Some(stmt) = self.parse_function_decl(true) {
                            self.ast.push(ASTNode::Statement(stmt));
                        }
                    } else if let Some(stmt) = self.parse_local_assignment() {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Return => {
                    self.next();
                    if let Some(stmt) = self.parse_return_statement() {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Identifier(ref name) => {
                    self.next();
                    let ident = name.clone();
                    if let Some(stmt) = self.parse_identifier_statement(ident) {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::If => {
                    self.next();
                    self.parse_if_statement();
                }
                _ => {
                    self.next();
                }
            }
        }

        &self.ast
    }

    pub fn parse_if_statement(&mut self) {
        let condition = match self.parse_expression(1) {
            Some(expr) => expr,
            None => {
                self.errors.push("Expected expression after 'if'".into());
                return;
            }
        };

        if !self.check_next(Token::Then) {
            self.errors.push("Expected 'then' after condition".into());
            return;
        }

        let then_block = self.parse_block_until(&[Token::ElseIf, Token::Else, Token::End]);
        let mut else_block = Vec::new();

        // Handle elseif chain as nested If nodes in else_block
        while matches!(self.peek(), Some(Token::ElseIf)) {
            self.next();
            let elseif_cond = match self.parse_expression(1) {
                Some(e) => e,
                None => {
                    self.errors
                        .push("Expected expression after 'elseif'".into());
                    return;
                }
            };
            if !self.check_next(Token::Then) {
                self.errors
                    .push("Expected 'then' after 'elseif' condition".into());
                return;
            }
            let elseif_body = self.parse_block_until(&[Token::ElseIf, Token::Else, Token::End]);
            else_block.push(ASTNode::Statement(StatementNode::If {
                condition: elseif_cond,
                then_block: elseif_body,
                else_block: Vec::new(),
            }));
        }

        // Handle final else
        if matches!(self.peek(), Some(Token::Else)) {
            self.next();
            else_block = self.parse_block_until(&[Token::End]);
        }

        if !self.check_next(Token::End) {
            self.errors
                .push("Expected 'end' to close 'if' statement".into());
            return;
        }

        self.ast.push(ASTNode::Statement(StatementNode::If {
            condition,
            then_block,
            else_block,
        }));
    }

    // Helper to parse statements until hitting one of the stop tokens
    fn parse_block_until(&mut self, stop_tokens: &[Token]) -> Vec<ASTNode> {
        let mut stmts = Vec::new();
        loop {
            let token = match self.peek() {
                Some(t) => t.clone(),
                None => break,
            };
            if stop_tokens.contains(&token) || token == Token::Eof {
                break;
            }

            match token {
                Token::Return => {
                    self.next();
                    if let Some(stmt) = self.parse_return_statement() {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Local => {
                    self.next();
                    if self.check_next(Token::Function) {
                        if let Some(stmt) = self.parse_function_decl(true) {
                            stmts.push(ASTNode::Statement(stmt));
                        }
                    } else if let Some(stmt) = self.parse_local_assignment() {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Function => {
                    self.next();
                    if let Some(stmt) = self.parse_function_decl(false) {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Identifier(ref name) => {
                    self.next();
                    let ident = name.clone();
                    if let Some(stmt) = self.parse_identifier_statement(ident) {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                Token::If => {
                    self.next();
                    self.parse_if_statement();
                }
                _ => {
                    self.next();
                }
            }
        }
        stmts
    }

    fn parse_local_assignment(&mut self) -> Option<StatementNode> {
        let mut targets = Vec::new();
        loop {
            match self.next() {
                Some(Token::Identifier(name)) => {
                    targets.push(ExpressionNode::Variable(name.clone()))
                }
                _ => {
                    self.errors
                        .push("Expected identifier in local declaration".into());
                    return None;
                }
            }

            if !self.check_next(Token::Comma) {
                break;
            }
        }

        let mut values = Vec::new();
        if self.check_next(Token::Assign) {
            loop {
                if let Some(expr) = self.parse_expression(1) {
                    values.push(expr);
                } else {
                    return None;
                }

                if !self.check_next(Token::Comma) {
                    break;
                }
            }
        }

        // Optional semicolon
        let _ = self.check_next(Token::Semicolon);

        Some(StatementNode::LocalAssignment { targets, values })
    }

    pub fn parse_function_decl(&mut self, is_local: bool) -> Option<StatementNode> {
        // Expect function name
        let name = if let Some(&Token::Identifier(ref name)) = self.next() {
            name.clone()
        } else {
            self.errors
                .push("Expected function name after 'function' keyword".to_string());
            return None;
        };

        // Expect '('
        if !self.check_next(Token::LeftParen) {
            self.errors
                .push("Expected '(' after function name".to_string());
            return None;
        }

        // Parse parameters
        let mut parameters = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                &Token::Identifier(ref param) => {
                    parameters.push(param.clone());
                    self.next(); // Consume parameter
                    if !self.check_next(Token::Comma) {
                        break;
                    }
                }
                &Token::RightParen => break,
                _ => {
                    self.errors
                        .push("Unexpected token in parameter list".to_string());
                    return None;
                }
            }
        }

        // Expect ')'
        if !self.check_next(Token::RightParen) {
            self.errors
                .push("Expected ')' after parameter list".to_string());
            return None;
        }

        let body_stmts = self.parse_block();
        let body = ASTNode::Statement(StatementNode::Block(body_stmts));
        if is_local {
            return Some(StatementNode::LocalFunctionDeclaration {
                name,
                parameters,
                body: Box::new(body),
            });
        }
        Some(StatementNode::FunctionDeclaration {
            name,
            parameters,
            body: Box::new(body),
        })
    }

    fn parse_identifier_statement(&mut self, first: String) -> Option<StatementNode> {
        // Try to parse as full expression with postfix (function call, indexing)
        let expr = self.parse_postfix(ExpressionNode::Variable(first))?;

        // Check if it's an assignment or expression statement
        if self.check_next(Token::Comma) {
            // Multiple assignment targets: x, y = ...
            let mut targets = vec![expr];
            loop {
                if let Some(e) = self.parse_postfix_expression() {
                    targets.push(e);
                } else {
                    return None;
                }
                if !self.check_next(Token::Comma) {
                    break;
                }
            }
            if !self.check_next(Token::Assign) {
                self.errors.push("Expected '=' in assignment".into());
                return None;
            }
            let values = self.parse_expression_list()?;
            let _ = self.check_next(Token::Semicolon);
            return Some(StatementNode::Assignment { targets, values });
        } else if self.check_next(Token::Assign) {
            // Single assignment: x = ...
            let values = self.parse_expression_list()?;
            let _ = self.check_next(Token::Semicolon);
            return Some(StatementNode::Assignment {
                targets: vec![expr],
                values,
            });
        } else {
            // Expression statement (function call)
            let _ = self.check_next(Token::Semicolon);
            return Some(StatementNode::ExpressionStatement(expr));
        }
    }

    fn parse_expression_list(&mut self) -> Option<Vec<ExpressionNode>> {
        let mut values = Vec::new();
        loop {
            if let Some(expr) = self.parse_expression(1) {
                values.push(expr);
            } else {
                return None;
            }
            if !self.check_next(Token::Comma) {
                break;
            }
        }
        Some(values)
    }

    fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut stmts = Vec::new();
        loop {
            let token = match self.peek() {
                Some(t) => t.clone(),
                None => break,
            };
            match token {
                Token::End => {
                    self.next();
                    break;
                }
                Token::Return => {
                    self.next();
                    if let Some(vals) = self.parse_return_tail() {
                        stmts.push(ASTNode::Statement(StatementNode::Return(vals)));
                    }
                }
                Token::Local => {
                    self.next();
                    if self.check_next(Token::Function) {
                        if let Some(stmt) = self.parse_function_decl(true) {
                            stmts.push(ASTNode::Statement(stmt));
                        }
                    } else if let Some(stmt) = self.parse_local_assignment() {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Function => {
                    self.next();
                    if let Some(stmt) = self.parse_function_decl(false) {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Identifier(ref name) => {
                    self.next();
                    let ident = name.clone();
                    if let Some(stmt) = self.parse_identifier_statement(ident) {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                _ => {
                    self.next();
                }
            }
        }
        stmts
    }

    fn parse_return_statement(&mut self) -> Option<StatementNode> {
        let values = self.parse_return_tail()?;
        Some(StatementNode::Return(values))
    }

    fn parse_return_tail(&mut self) -> Option<Vec<ExpressionNode>> {
        let mut values = Vec::new();
        if let Some(tok) = self.peek() {
            if matches!(tok, Token::End | Token::Semicolon) {
                return Some(values);
            }
        }
        loop {
            if let Some(expr) = self.parse_expression(1) {
                values.push(expr);
            } else {
                return None;
            }
            if !self.check_next(Token::Comma) {
                break;
            }
        }
        let _ = self.check_next(Token::Semicolon);
        Some(values)
    }

    fn parse_expression(&mut self, min_prec: u8) -> Option<ExpressionNode> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Some(t) if Self::is_binary_op(t) => t.clone(),
                _ => break,
            };
            let prec = Self::precedence(&op);
            let right_assoc = Self::right_associative(&op);
            if prec < min_prec {
                break;
            }
            self.next();
            let next_min = if right_assoc { prec } else { prec + 1 };
            let right = self.parse_expression(next_min)?;
            left = ExpressionNode::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<ExpressionNode> {
        if let Some(tok) = self.peek() {
            if matches!(tok, Token::Minus | Token::Not | Token::Length) {
                let op = tok.clone();
                self.next();
                let operand = self.parse_unary()?;
                return Some(ExpressionNode::UnaryOp {
                    operator: op,
                    operand: Box::new(operand),
                });
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<ExpressionNode> {
        let base = match self.next()? {
            Token::Identifier(name) => ExpressionNode::Variable(name.clone()),
            Token::Integer(v) => ExpressionNode::Literal(LiteralNode::Number(*v as f64)),
            Token::Number(f) => ExpressionNode::Literal(LiteralNode::Number(*f)),
            Token::StringLiteral(s) => ExpressionNode::Literal(LiteralNode::String(s.clone())),
            Token::True => ExpressionNode::Literal(LiteralNode::Boolean(true)),
            Token::False => ExpressionNode::Literal(LiteralNode::Boolean(false)),
            Token::Nil => ExpressionNode::Literal(LiteralNode::Nil),
            Token::LeftParen => {
                let expr = self.parse_expression(1)?;
                if !self.check_next(Token::RightParen) {
                    self.errors.push("Expected ')'".into());
                }
                return self.parse_postfix(expr);
            }
            Token::LeftBrace => return self.parse_table_constructor(),
            _ => {
                self.errors.push("Unexpected token in expression".into());
                return None;
            }
        };
        self.parse_postfix(base)
    }

    fn parse_postfix_expression(&mut self) -> Option<ExpressionNode> {
        let base = match self.next()? {
            Token::Identifier(name) => ExpressionNode::Variable(name.clone()),
            Token::LeftParen => {
                let expr = self.parse_expression(1)?;
                if !self.check_next(Token::RightParen) {
                    self.errors.push("Expected ')'".into());
                }
                expr
            }
            _ => {
                self.errors.push("Expected expression".into());
                return None;
            }
        };
        self.parse_postfix(base)
    }

    fn parse_postfix(&mut self, mut base: ExpressionNode) -> Option<ExpressionNode> {
        loop {
            match self.peek()? {
                Token::LeftParen => {
                    // Function call
                    self.next();
                    let arguments = if matches!(self.peek(), Some(Token::RightParen)) {
                        Vec::new()
                    } else {
                        self.parse_expression_list()?
                    };
                    if !self.check_next(Token::RightParen) {
                        self.errors.push("Expected ')' after arguments".into());
                        return None;
                    }
                    // Extract function name from base (if it's a Variable or Index)
                    base = ExpressionNode::FunctionCall {
                        function: Box::new(base),
                        arguments,
                    };
                }
                Token::LeftBracket => {
                    // Index with brackets: t[key]
                    self.next();
                    let index = self.parse_expression(1)?;
                    if !self.check_next(Token::RightBracket) {
                        self.errors.push("Expected ']'".into());
                        return None;
                    }
                    base = ExpressionNode::Index {
                        table: Box::new(base),
                        index: Box::new(index),
                    };
                }
                Token::Dot => {
                    // Index with dot: t.key
                    self.next();
                    if let Some(Token::Identifier(key)) = self.next() {
                        let key_expr = ExpressionNode::Literal(LiteralNode::String(key.clone()));
                        base = ExpressionNode::Index {
                            table: Box::new(base),
                            index: Box::new(key_expr),
                        };
                    } else {
                        self.errors.push("Expected identifier after '.'".into());
                        return None;
                    }
                }
                _ => break,
            }
        }
        Some(base)
    }

    fn parse_table_constructor(&mut self) -> Option<ExpressionNode> {
        let mut entries = Vec::new();
        loop {
            match self.peek()? {
                Token::RightBrace => {
                    self.next();
                    break;
                }
                Token::LeftBracket => {
                    // [key] = value
                    self.next();
                    let key = self.parse_expression(1)?;
                    if !self.check_next(Token::RightBracket) {
                        self.errors.push("Expected ']'".into());
                        return None;
                    }
                    if !self.check_next(Token::Assign) {
                        self.errors.push("Expected '=' after table key".into());
                        return None;
                    }
                    let value = self.parse_expression(1)?;
                    entries.push((Some(key), value));
                }
                Token::Identifier(_) => {
                    // Could be key = value or just value
                    let start_expr = self.parse_expression(1)?;
                    if self.check_next(Token::Assign) {
                        // It was a key
                        let value = self.parse_expression(1)?;
                        entries.push((Some(start_expr), value));
                    } else {
                        // It was just a value (array-style entry)
                        entries.push((None, start_expr));
                    }
                }
                _ => {
                    // Expression value (array-style)
                    let value = self.parse_expression(1)?;
                    entries.push((None, value));
                }
            }
            if !self.check_next(Token::Comma) && !self.check_next(Token::Semicolon) {
                if !matches!(self.peek(), Some(Token::RightBrace)) {
                    self.errors
                        .push("Expected ',' or '}' in table constructor".into());
                    return None;
                }
            }
        }
        Some(ExpressionNode::TableConstructor { entries })
    }

    fn is_binary_op(token: &Token) -> bool {
        matches!(
            token,
            Token::Plus
                | Token::Minus
                | Token::Multiply
                | Token::Divide
                | Token::Modulus
                | Token::Power
                | Token::Concat
                | Token::Equal
                | Token::NotEqual
                | Token::LessEqual
                | Token::GreaterEqual
                | Token::LessThan
                | Token::GreaterThan
                | Token::And
                | Token::Or
                | Token::FloorDivide
        )
    }

    fn precedence(token: &Token) -> u8 {
        match token {
            Token::Or => 1,
            Token::And => 2,
            Token::Equal
            | Token::NotEqual
            | Token::LessEqual
            | Token::GreaterEqual
            | Token::LessThan
            | Token::GreaterThan => 3,
            Token::Concat => 4,
            Token::Plus | Token::Minus => 5,
            Token::Multiply | Token::Divide | Token::Modulus | Token::FloorDivide => 6,
            Token::Power => 8,
            _ => 0,
        }
    }

    fn right_associative(token: &Token) -> bool {
        matches!(token, Token::Power | Token::Concat)
    }

    // Private Little Helpers //
    #[inline]
    pub fn next(&mut self) -> Option<&Token> {
        self.tokens.next()
    }
    #[inline]
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek().copied()
    }
    #[inline]
    fn check_next(&mut self, expected: Token) -> bool {
        if let Some(token) = self.peek() {
            if token == &expected {
                self.next();
                return true;
            }
        }
        false
    }
}
