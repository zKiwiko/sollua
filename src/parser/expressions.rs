use crate::ast::*;
use crate::lexer::Token;
use crate::parser::Parser;

impl<'src> Parser<'src> {
    #[inline(always)]
    pub(super) fn parse_expression(&mut self, min_prec: u8) -> Option<ExpressionNode<'src>> {
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

    #[inline(always)]
    fn parse_unary(&mut self) -> Option<ExpressionNode<'src>> {
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

    #[inline(always)]
    fn parse_primary(&mut self) -> Option<ExpressionNode<'src>> {
        let base = match self.next()? {
            Token::Identifier(name) => ExpressionNode::Variable(name),
            Token::Integer(v) => ExpressionNode::Literal(LiteralNode::Number(*v as f64)),
            Token::Number(f) => ExpressionNode::Literal(LiteralNode::Number(*f)),
            Token::StringLiteral(s) => ExpressionNode::Literal(LiteralNode::String(s)),
            Token::True => ExpressionNode::Literal(LiteralNode::Boolean(true)),
            Token::False => ExpressionNode::Literal(LiteralNode::Boolean(false)),
            Token::Nil => ExpressionNode::Literal(LiteralNode::Nil),
            Token::VarArgs => ExpressionNode::VarArg,
            Token::Function => return self.parse_anonymous_function(),
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

    #[inline(always)]
    pub(super) fn parse_postfix_expression(&mut self) -> Option<ExpressionNode<'src>> {
        let base = match self.next()? {
            Token::Identifier(name) => ExpressionNode::Variable(name),
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

    #[inline(always)]
    pub(super) fn parse_postfix(&mut self, mut base: ExpressionNode<'src>) -> Option<ExpressionNode<'src>> {
        loop {
            match self.peek()? {
                Token::LeftParen => {
                    // Regular function call: func(args)
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
                    base = ExpressionNode::FunctionCall {
                        function: Box::new(base),
                        method: None,
                        arguments,
                    };
                }
                Token::Colon => {
                    // Method call: obj:method(args)
                    self.next();
                    let method = if let Some(Token::Identifier(name)) = self.next() {
                        *name
                    } else {
                        self.errors.push("Expected method name after ':'".into());
                        return None;
                    };

                    // Parse arguments - can be (), table, or string
                    let arguments = if self.check_next(Token::LeftParen) {
                        let args = if matches!(self.peek(), Some(Token::RightParen)) {
                            Vec::new()
                        } else {
                            self.parse_expression_list()?
                        };
                        if !self.check_next(Token::RightParen) {
                            self.errors.push("Expected ')' after arguments".into());
                            return None;
                        }
                        args
                    } else if matches!(self.peek(), Some(Token::LeftBrace)) {
                        // Table constructor as argument
                        self.next(); // consume {
                        vec![self.parse_table_constructor()?]
                    } else if let Some(Token::StringLiteral(s)) = self.peek() {
                        // String literal as argument
                        let s = *s;
                        self.next();
                        vec![ExpressionNode::Literal(LiteralNode::String(s))]
                    } else {
                        self.errors
                            .push("Expected arguments after method name".into());
                        return None;
                    };

                    base = ExpressionNode::FunctionCall {
                        function: Box::new(base),
                        method: Some(method),
                        arguments,
                    };
                }
                Token::LeftBrace => {
                    // Table constructor as function argument: func{...}
                    self.next(); // consume {
                    let table = self.parse_table_constructor()?;
                    base = ExpressionNode::FunctionCall {
                        function: Box::new(base),
                        method: None,
                        arguments: vec![table],
                    };
                }
                Token::StringLiteral(s) => {
                    // String literal as function argument: func"..."
                    let s = *s;
                    self.next();
                    base = ExpressionNode::FunctionCall {
                        function: Box::new(base),
                        method: None,
                        arguments: vec![ExpressionNode::Literal(LiteralNode::String(s))],
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
                        let key_expr = ExpressionNode::Literal(LiteralNode::String(key));
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
    #[inline(always)]
    fn parse_table_constructor(&mut self) -> Option<ExpressionNode<'src>> {
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

    #[inline(always)]
    fn parse_anonymous_function(&mut self) -> Option<ExpressionNode<'src>> {
        // Expect '('
        if !self.check_next(Token::LeftParen) {
            self.errors
                .push("Expected '(' after 'function' keyword".into());
            return None;
        }

        // Parse parameters
        let mut parameters = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                &Token::Identifier(ref param) => {
                    parameters.push(*param);
                    self.next();
                    if !self.check_next(Token::Comma) {
                        break;
                    }
                }
                &Token::VarArgs => {
                    parameters.push("...");
                    self.next();
                    break;
                }
                &Token::RightParen => break,
                _ => {
                    self.errors
                        .push("Unexpected token in parameter list".into());
                    return None;
                }
            }
        }

        if !self.check_next(Token::RightParen) {
            self.errors.push("Expected ')' after parameter list".into());
            return None;
        }

        let body_stmts = self.parse_block();
        let body = ASTNode::Statement(StatementNode::Block(body_stmts));

        Some(ExpressionNode::AnonymousFunction {
            parameters,
            body: Box::new(body),
        })
    }
}
