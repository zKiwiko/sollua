use sollua::lexer::Lexer;
use sollua::lexer::Token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        let source = "local x = -1; local y = 1 - var;";
        let mut lexer = Lexer::new(source);

        let tokens: Vec<_> = lexer.collect();

        println!("{:?}", tokens);
    }

    #[test]
    fn test_attribute() {
        let source = "local <const> x = 42;";
        let mut lexer = Lexer::new(source);

        let tokens: Vec<_> = lexer.collect();

        println!("{:?}", tokens);

        assert_eq!(
            tokens.contains(&Token::Attribute("const".to_string())),
            true
        );
        assert_eq!(tokens.contains(&Token::Local), true);
        assert_eq!(tokens.contains(&Token::Identifier("x".to_string())), true);
        assert_eq!(tokens.contains(&Token::Assign), true);
        assert_eq!(tokens.contains(&Token::Integer(42)), true);
        assert_eq!(tokens.contains(&Token::Semicolon), true);
    }

    #[test]
    fn test_keywords() {
        let source = "and break do else elseif end false for function goto if in local nil not or repeat return then true until while";
        let mut lexer = Lexer::new(source);

        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens.contains(&Token::If), true);
        assert_eq!(tokens.contains(&Token::ElseIf), true);
        assert_eq!(tokens.contains(&Token::End), true);
        assert_eq!(tokens.contains(&Token::False), true);
        assert_eq!(tokens.contains(&Token::For), true);
        assert_eq!(tokens.contains(&Token::Function), true);
        assert_eq!(tokens.contains(&Token::Goto), true);
        assert_eq!(tokens.contains(&Token::In), true);
        assert_eq!(tokens.contains(&Token::Local), true);
        assert_eq!(tokens.contains(&Token::Nil), true);
        assert_eq!(tokens.contains(&Token::Not), true);
        assert_eq!(tokens.contains(&Token::Or), true);
        assert_eq!(tokens.contains(&Token::Repeat), true);
        assert_eq!(tokens.contains(&Token::Return), true);
        assert_eq!(tokens.contains(&Token::Then), true);
        assert_eq!(tokens.contains(&Token::True), true);
        assert_eq!(tokens.contains(&Token::Until), true);
        assert_eq!(tokens.contains(&Token::While), true);
    }

    #[test]
    fn test_operators() {
        let source = ": ; + - * / % ^ # == ~= <= >= << >> < > = ( ) { } [ ] ; : , . .. ...";
        let mut lexer = Lexer::new(source);

        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens.contains(&Token::Plus), true);
        assert_eq!(tokens.contains(&Token::Semicolon), true);
        assert_eq!(tokens.contains(&Token::Colon), true);
        assert_eq!(tokens.contains(&Token::Comma), true);
        assert_eq!(tokens.contains(&Token::Dot), true);
        assert_eq!(tokens.contains(&Token::ShiftLeft), true);
        assert_eq!(tokens.contains(&Token::ShiftRight), true);
        assert_eq!(tokens.contains(&Token::Minus), true);
        assert_eq!(tokens.contains(&Token::Multiply), true);
        assert_eq!(tokens.contains(&Token::Divide), true);
        assert_eq!(tokens.contains(&Token::Modulus), true);
        assert_eq!(tokens.contains(&Token::Power), true);
        assert_eq!(tokens.contains(&Token::Length), true);
        assert_eq!(tokens.contains(&Token::Equal), true);
        assert_eq!(tokens.contains(&Token::NotEqual), true);
        assert_eq!(tokens.contains(&Token::LessEqual), true);
        assert_eq!(tokens.contains(&Token::GreaterEqual), true);
        assert_eq!(tokens.contains(&Token::LessThan), true);
        assert_eq!(tokens.contains(&Token::GreaterThan), true);
        assert_eq!(tokens.contains(&Token::Assign), true);
        assert_eq!(tokens.contains(&Token::LeftParen), true);
        assert_eq!(tokens.contains(&Token::RightParen), true);
        assert_eq!(tokens.contains(&Token::LeftBrace), true);
        assert_eq!(tokens.contains(&Token::RightBrace), true);
        assert_eq!(tokens.contains(&Token::LeftBracket), true);
        assert_eq!(tokens.contains(&Token::RightBracket), true);
        assert_eq!(tokens.contains(&Token::Semicolon), true);
        assert_eq!(tokens.contains(&Token::Colon), true);
        assert_eq!(tokens.contains(&Token::Comma), true);
        assert_eq!(tokens.contains(&Token::Dot), true);
        assert_eq!(tokens.contains(&Token::Concat), true);
        assert_eq!(tokens.contains(&Token::VarArgs), true);
    }

    #[test]
    fn test_identifiers() {
        let source = "Foo Bar _Foo _Bar _123 abc123";
        let mut lexer = Lexer::new(source);

        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens.contains(&Token::Identifier("Foo".to_string())), true);
        assert_eq!(tokens.contains(&Token::Identifier("Bar".to_string())), true);
        assert_eq!(
            tokens.contains(&Token::Identifier("_Foo".to_string())),
            true
        );
        assert_eq!(
            tokens.contains(&Token::Identifier("_Bar".to_string())),
            true
        );
        assert_eq!(
            tokens.contains(&Token::Identifier("_123".to_string())),
            true
        );
        assert_eq!(
            tokens.contains(&Token::Identifier("abc123".to_string())),
            true
        );
    }

    #[test]
    fn test_numbers() {
        let source = "123 0x1A3F -456 0xFF -0x10 3.14 -0.001 1e10 -2.5e-3 0XABC";
        let mut lexer = Lexer::new(source);

        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens.contains(&Token::Integer(123)), true);
        assert_eq!(tokens.contains(&Token::Integer(0x1A3F)), true);
        assert_eq!(tokens.contains(&Token::Integer(456)), true);
        assert_eq!(tokens.contains(&Token::Integer(0xFF)), true);
        assert_eq!(tokens.contains(&Token::Integer(0x10)), true);
        assert_eq!(tokens.contains(&Token::Number(3.14)), true);
        assert_eq!(tokens.contains(&Token::Number(0.001)), true);
        assert_eq!(tokens.contains(&Token::Number(1e10)), true);
    }

    #[test]
    fn test_string_literals() {
        let source = r#""Hello, World!" 'Single quoted string' "String with escape \n characters" [[long]] "#;
        let mut lexer = Lexer::new(source);

        let tokens: Vec<_> = lexer.collect();

        assert_eq!(
            tokens.contains(&Token::StringLiteral("Hello, World!".to_string())),
            true
        );
        assert_eq!(
            tokens.contains(&Token::StringLiteral("Single quoted string".to_string())),
            true
        );
        assert_eq!(
            tokens.contains(&Token::StringLiteral(
                "String with escape \n characters".to_string()
            )),
            true
        );
        assert_eq!(
            tokens.contains(&Token::StringLiteral("long".to_string())),
            true
        );
    }

    #[test]
    fn test_comments() {
        let source = r#"
-- This is a line comment
--[[
This is a block comment
that spans multiple lines
]]
"#;
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_labels() {
        let source = "::start:: local x = 10; ::end::";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens.contains(&Token::Label("start".to_string())), true);
        assert_eq!(tokens.contains(&Token::Label("end".to_string())), true);
    }
}
