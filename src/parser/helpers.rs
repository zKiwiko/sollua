use crate::lexer::Token;
use crate::parser::Parser;

#[inline]
pub(super) fn matches_any<'src>(token: &Token<'src>, variants: &[Token<'src>]) -> bool {
    variants.iter().any(|v| std::mem::discriminant(token) == std::mem::discriminant(v))
}

impl<'src> Parser<'src> {
    #[inline]
    pub fn next(&mut self) -> Option<&Token<'src>> {
        self.tokens.next()
    }

    #[inline]
    pub(super) fn peek(&mut self) -> Option<&Token<'src>> {
        self.tokens.peek().copied()
    }

    #[inline]
    pub(super) fn check_next(&mut self, expected: Token<'src>) -> bool {
        if let Some(token) = self.peek() {
            if std::mem::discriminant(token) == std::mem::discriminant(&expected) {
                self.next();
                return true;
            }
        }
        false
    }

    #[inline(always)]
    pub(super) fn is_binary_op(token: &Token<'src>) -> bool {
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

    #[inline(always)]
    pub(super) fn precedence(token: &Token<'src>) -> u8 {
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

    #[inline(always)]
    pub(super) fn right_associative(token: &Token<'src>) -> bool {
        matches!(token, Token::Power | Token::Concat)
    }

    #[inline(always)]
    pub(super) fn parse_expression_list(&mut self) -> Option<Vec<crate::ast::ExpressionNode<'src>>> {
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
