use sollua::lexer::Lexer;
use sollua::parser::Parser;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1000_lines() {
        // Ensure this is run in release mode
        assert!(
            cfg!(not(debug_assertions)),
            "Run this test with: cargo test --release -- --nocapture"
        );

        let source = std::fs::read_to_string("lua/onethousand.lua").unwrap();
        let start = Instant::now();
        let mut lexer = Lexer::new(&source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(&source, &tokens);
        parser.parse();
        let elapsed = start.elapsed();
        println!("parse elapsed: {:.3?}", elapsed);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
    }

    #[test]
    fn test_2500_lines() {
        // Ensure this is run in release mode
        assert!(
            cfg!(not(debug_assertions)),
            "Run this test with: cargo test --release -- --nocapture"
        );

        let source = std::fs::read_to_string("lua/twothousand.lua").unwrap();
        let start = Instant::now();
        let mut lexer = Lexer::new(&source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(&source, &tokens);
        parser.parse();
        let elapsed = start.elapsed();
        println!("parse elapsed: {:.3?}", elapsed);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
    }

    #[test]
    fn test_5000_lines() {
        // Ensure this is run in release mode
        assert!(
            cfg!(not(debug_assertions)),
            "Run this test with: cargo test --release -- --nocapture"
        );

        let source = std::fs::read_to_string("lua/fivethousand.lua").unwrap();
        let start = Instant::now();
        let mut lexer = Lexer::new(&source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(&source, &tokens);
        parser.parse();
        let elapsed = start.elapsed();
        println!("parse elapsed: {:.3?}", elapsed);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
    }
}
