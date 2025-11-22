use sollua::lexer::Lexer;
use sollua::parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_statement() {
        let source = "if x > 0 then y = 1; else y = -1; end";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
    }
    #[test]
    fn test_function_declaration() {
        let source =
            "function add(a, b) return a + b end local function sub(a, b) return a - b end";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
    }

    #[test]
    fn test_local_assignment() {
        let source = "local x, y = 10, 20;";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
        assert_eq!(ast.len(), 1, "Expected 1 AST node, found {}", ast.len());
    }

    #[test]
    fn test_function_call() {
        let source = "print(\"Hello\", 42); foo(x + 1);";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
        assert_eq!(ast.len(), 2);
    }

    #[test]
    fn test_table_constructor() {
        let source = "local t = {1, 2, x = 3, [\"key\"] = 4};";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
    }

    #[test]
    fn test_indexing() {
        let source = "x = t.field; y = t[key]; z = t[1];";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
        assert_eq!(ast.len(), 3);
    }

    #[test]
    fn test_complex_expression() {
        let source = "local result = math.sqrt(a^2 + b^2);";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
    }

    #[test]
    fn test_while_loop() {
        let source = "while n > 0 do n = n - 1; end";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
    }

    #[test]
    fn test_for_loop() {
        let source = "for i = 1, 10 do print(i); end";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
        assert_eq!(ast.len(), 1, "Expected 1 AST node, found {}", ast.len());
    }

    #[test]
    fn test_generic_for_loop() {
        let source = "for k, v in pairs(t) do print(k, v); end";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
        assert_eq!(ast.len(), 1, "Expected 1 AST node, found {}", ast.len());
    }

    #[test]
    fn test_repeat_until_loop() {
        let source = "repeat x = x + 1; until x >= 10;";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
        assert_eq!(ast.len(), 1, "Expected 1 AST node, found {}", ast.len());
    }

    #[test]
    fn test_do_block() {
        let source = "do local x = 10; print(x); end";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
        assert_eq!(ast.len(), 1, "Expected 1 AST node, found {}", ast.len());
    }

    #[test]
    fn test_goto_statement() {
        let source = "::start:: print('Hello'); goto start;";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(source, &tokens);
        let ast = parser.parse().clone();

        println!("AST: \n{:#?}", ast);

        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );
        assert_eq!(
            ast.len(),
            3,
            "Expected 3 AST nodes (label, statement, goto), found {}",
            ast.len()
        );
    }
}
