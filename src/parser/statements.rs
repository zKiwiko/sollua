use crate::ast::*;
use crate::lexer::Token;
use crate::parser::Parser;

impl<'src> Parser<'src> {
    pub fn parse_label_statement(&mut self, label: &'src str) -> StatementNode<'src> {
        let _ = self.check_next(Token::Semicolon);
        StatementNode::Label(label)
    }

    pub fn parse_goto_statement(&mut self) -> Option<StatementNode<'src>> {
        let label = if let Some(&Token::Identifier(ref name)) = self.next() {
            *name
        } else {
            self.errors
                .push("Expected label name after 'goto' keyword".into());
            return None;
        };

        let _ = self.check_next(Token::Semicolon);

        Some(StatementNode::Goto(label))
    }

    pub fn parse_do_block(&mut self) -> Option<StatementNode<'src>> {
        let body = self.parse_block_until(&[Token::End]);

        if !self.check_next(Token::End) {
            self.errors
                .push("Expected 'end' to close 'do' block".into());
            return None;
        }

        Some(StatementNode::DoBlock { body })
    }

    pub fn parse_repeat_statement(&mut self) -> Option<StatementNode<'src>> {
        let body = self.parse_block_until(&[Token::Until]);

        if !self.check_next(Token::Until) {
            self.errors
                .push("Expected 'until' to close 'repeat' statement".into());
            return None;
        }

        let condition = match self.parse_expression(1) {
            Some(expr) => expr,
            None => {
                self.errors.push("Expected expression after 'until'".into());
                return None;
            }
        };

        Some(StatementNode::Repeat { body, condition })
    }

    pub fn parse_for_statement(&mut self) -> Option<StatementNode<'src>> {
        let first_var = if let Some(&Token::Identifier(ref name)) = self.next() {
            *name
        } else {
            self.errors
                .push("Expected variable name in 'for' statement".into());
            return None;
        };

        if self.check_next(Token::Assign) {
            return self.parse_numeric_for(first_var);
        } else {
            return self.parse_generic_for(first_var);
        }
    }

    fn parse_numeric_for(&mut self, var_name: &'src str) -> Option<StatementNode<'src>> {
        let start_expr = match self.parse_expression(1) {
            Some(expr) => expr,
            None => {
                self.errors
                    .push("Expected start expression in 'for' statement".into());
                return None;
            }
        };

        if !self.check_next(Token::Comma) {
            self.errors
                .push("Expected ',' after start expression in 'for' statement".into());
            return None;
        }

        let end_expr = match self.parse_expression(1) {
            Some(expr) => expr,
            None => {
                self.errors
                    .push("Expected end expression in 'for' statement".into());
                return None;
            }
        };

        let step_expr = if self.check_next(Token::Comma) {
            match self.parse_expression(1) {
                Some(expr) => Some(expr),
                None => {
                    self.errors
                        .push("Expected step expression in 'for' statement".into());
                    return None;
                }
            }
        } else {
            None
        };

        if !self.check_next(Token::Do) {
            self.errors.push("Expected 'do' in 'for' statement".into());
            return None;
        }

        let body = self.parse_block_until(&[Token::End]);

        if !self.check_next(Token::End) {
            self.errors
                .push("Expected 'end' to close 'for' statement".into());
            return None;
        }

        Some(StatementNode::ForNumeric {
            variable: var_name,
            start: start_expr,
            end: end_expr,
            step: step_expr,
            body,
        })
    }

    fn parse_generic_for(&mut self, first_var: &'src str) -> Option<StatementNode<'src>> {
        let mut variables = vec![first_var];
        while self.check_next(Token::Comma) {
            if let Some(&Token::Identifier(ref name)) = self.next() {
                variables.push(*name);
            } else {
                self.errors.push("Expected variable name after ','".into());
                return None;
            }
        }

        if !self.check_next(Token::In) {
            self.errors
                .push("Expected 'in' in generic 'for' loop".into());
            return None;
        }

        let expressions = self.parse_expression_list()?;

        if !self.check_next(Token::Do) {
            self.errors.push("Expected 'do' in 'for' statement".into());
            return None;
        }

        let body = self.parse_block_until(&[Token::End]);

        if !self.check_next(Token::End) {
            self.errors
                .push("Expected 'end' to close 'for' statement".into());
            return None;
        }

        Some(StatementNode::ForGeneric {
            variables,
            expressions,
            body,
        })
    }

    pub fn parse_while_statement(&mut self) -> Option<StatementNode<'src>> {
        let condition = match self.parse_expression(1) {
            Some(expr) => expr,
            None => {
                self.errors.push("Expected expression after 'while'".into());
                return None;
            }
        };

        if !self.check_next(Token::Do) {
            self.errors.push("Expected 'do' after condition".into());
            return None;
        }

        let body = self.parse_block_until(&[Token::End]);

        if !self.check_next(Token::End) {
            self.errors
                .push("Expected 'end' to close 'while' statement".into());
            return None;
        }

        Some(StatementNode::While { condition, body })
    }

    pub fn parse_if_statement(&mut self) -> Option<StatementNode<'src>> {
        let condition = match self.parse_expression(1) {
            Some(expr) => expr,
            None => {
                self.errors.push("Expected expression after 'if'".into());
                return None;
            }
        };

        if !self.check_next(Token::Then) {
            self.errors.push("Expected 'then' after condition".into());
            return None;
        }

        let then_block = self.parse_block_until(&[Token::ElseIf, Token::Else, Token::End]);
        let mut else_block = Vec::new();

        while matches!(self.peek(), Some(Token::ElseIf)) {
            self.next();
            let elseif_cond = match self.parse_expression(1) {
                Some(e) => e,
                None => {
                    self.errors
                        .push("Expected expression after 'elseif'".into());
                    return None;
                }
            };
            if !self.check_next(Token::Then) {
                self.errors
                    .push("Expected 'then' after 'elseif' condition".into());
                return None;
            }
            let elseif_body = self.parse_block_until(&[Token::ElseIf, Token::Else, Token::End]);
            else_block.push(ASTNode::Statement(StatementNode::If {
                condition: elseif_cond,
                then_block: elseif_body,
                else_block: Vec::new(),
            }));
        }

        if matches!(self.peek(), Some(Token::Else)) {
            self.next();
            else_block = self.parse_block_until(&[Token::End]);
        }

        if !self.check_next(Token::End) {
            let peek_tok = self.peek().cloned();
            self.errors.push(format!(
                "Expected 'end' to close 'if' statement, found {:?}",
                peek_tok
            ));
            return None;
        }

        Some(StatementNode::If {
            condition,
            then_block,
            else_block,
        })
    }

    pub(super) fn parse_block_until(&mut self, stop_tokens: &[Token<'src>]) -> Vec<ASTNode<'src>> {
        let mut stmts = Vec::new();
        loop {
            let token = match self.peek() {
                Some(t)
                    if super::helpers::matches_any(t, stop_tokens) || matches!(t, Token::Eof) =>
                {
                    break;
                }
                Some(t) => t.clone(),
                None => break,
            };

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
                Token::Identifier(name) => {
                    self.next();
                    if let Some(stmt) = self.parse_identifier_statement(name) {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                Token::If => {
                    self.next();
                    if let Some(stmt) = self.parse_if_statement() {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Label(name) => {
                    self.next();
                    let stmt = self.parse_label_statement(name);
                    stmts.push(ASTNode::Statement(stmt));
                }
                Token::Goto => {
                    self.next();
                    if let Some(stmt) = self.parse_goto_statement() {
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

    pub(super) fn parse_local_assignment(&mut self) -> Option<StatementNode<'src>> {
        let mut targets = Vec::new();
        loop {
            let var_name = match self.next() {
                Some(Token::Identifier(name)) => ExpressionNode::Variable(name),
                _ => {
                    self.errors
                        .push("Expected identifier in local declaration".into());
                    return None;
                }
            };

            let attribute = if let Some(Token::Attribute(attr)) = self.peek() {
                if attr == &"const" || attr == &"close" {
                    let attr_val = *attr;
                    self.next();
                    Some(attr_val)
                } else {
                    None
                }
            } else {
                None
            };

            targets.push((var_name, attribute));

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

        let _ = self.check_next(Token::Semicolon);

        Some(StatementNode::LocalAssignment { targets, values })
    }

    pub fn parse_function_decl(&mut self, is_local: bool) -> Option<StatementNode<'src>> {
        let first_name = if let Some(&Token::Identifier(ref name)) = self.next() {
            *name
        } else {
            self.errors
                .push("Expected function name after 'function' keyword".to_string());
            return None;
        };

        if is_local {
            if !self.check_next(Token::LeftParen) {
                self.errors
                    .push("Expected '(' after function name".to_string());
                return None;
            }

            let parameters = self.parse_parameter_list()?;

            if !self.check_next(Token::RightParen) {
                self.errors
                    .push("Expected ')' after parameter list".to_string());
                return None;
            }

            let body_stmts = self.parse_block();
            let body = ASTNode::Statement(StatementNode::Block(body_stmts));
            return Some(StatementNode::LocalFunctionDeclaration {
                name: first_name,
                parameters,
                body: Box::new(body),
            });
        }

        let mut name_path = vec![first_name];
        let mut is_method = false;

        loop {
            if self.check_next(Token::Dot) {
                if let Some(&Token::Identifier(ref name)) = self.next() {
                    name_path.push(*name);
                } else {
                    self.errors.push("Expected identifier after '.'".into());
                    return None;
                }
            } else if self.check_next(Token::Colon) {
                if let Some(&Token::Identifier(ref name)) = self.next() {
                    name_path.push(*name);
                    is_method = true;
                    break;
                } else {
                    self.errors.push("Expected identifier after ':'".into());
                    return None;
                }
            } else {
                break;
            }
        }

        if !self.check_next(Token::LeftParen) {
            self.errors
                .push("Expected '(' after function name".to_string());
            return None;
        }

        let mut parameters = self.parse_parameter_list()?;

        if is_method {
            parameters.insert(0, "self");
        }

        if !self.check_next(Token::RightParen) {
            self.errors
                .push("Expected ')' after parameter list".to_string());
            return None;
        }

        let body_stmts = self.parse_block();
        let body = ASTNode::Statement(StatementNode::Block(body_stmts));
        Some(StatementNode::FunctionDeclaration {
            name_path,
            is_method,
            parameters,
            body: Box::new(body),
        })
    }

    fn parse_parameter_list(&mut self) -> Option<Vec<&'src str>> {
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
                        .push("Unexpected token in parameter list".to_string());
                    return None;
                }
            }
        }
        Some(parameters)
    }

    pub(super) fn parse_identifier_statement(
        &mut self,
        first: &'src str,
    ) -> Option<StatementNode<'src>> {
        let expr = self.parse_postfix(ExpressionNode::Variable(first))?;

        if self.check_next(Token::Comma) {
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
            let values = self.parse_expression_list()?;
            let _ = self.check_next(Token::Semicolon);
            return Some(StatementNode::Assignment {
                targets: vec![expr],
                values,
            });
        } else {
            let _ = self.check_next(Token::Semicolon);
            return Some(StatementNode::ExpressionStatement(expr));
        }
    }

    pub(super) fn parse_block(&mut self) -> Vec<ASTNode<'src>> {
        let mut stmts = Vec::new();
        loop {
            let token_kind = match self.peek() {
                Some(Token::End) => {
                    self.next();
                    break;
                }
                Some(Token::Identifier(name)) => {
                    let name_copy = *name;
                    self.next();
                    if let Some(stmt) = self.parse_identifier_statement(name_copy) {
                        stmts.push(ASTNode::Statement(stmt));
                    }
                    continue;
                }
                Some(token) => token.clone(),
                None => break,
            };

            match token_kind {
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
                _ => {
                    self.next();
                }
            }
        }
        stmts
    }

    pub(super) fn parse_return_statement(&mut self) -> Option<StatementNode<'src>> {
        let values = self.parse_return_tail()?;
        Some(StatementNode::Return(values))
    }

    pub(super) fn parse_return_tail(&mut self) -> Option<Vec<ExpressionNode<'src>>> {
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
}
