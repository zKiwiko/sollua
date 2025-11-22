use crate::lexer::Token;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    #[inline]
    pub fn next(&mut self) -> Option<&Token> {
        self.tokens.next()
    }

    #[inline]
    pub(super) fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek().copied()
    }

    #[inline]
    pub(super) fn check_next(&mut self, expected: Token) -> bool {
        if let Some(token) = self.peek() {
            if token == &expected {
                self.next();
                return true;
            }
        }
        false
    }

    pub(super) fn is_binary_op(token: &Token) -> bool {
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

    pub(super) fn precedence(token: &Token) -> u8 {
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

    pub(super) fn right_associative(token: &Token) -> bool {
        matches!(token, Token::Power | Token::Concat)
    }

    pub(super) fn parse_expression_list(&mut self) -> Option<Vec<crate::ast::ExpressionNode>> {
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
}
