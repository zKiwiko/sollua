use logos::{Lexer as LogosLexer, Logos};

mod helpers;
use helpers::unescape;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // Keywords
    #[token("and")]
    And,
    #[token("break")]
    Break,
    #[token("do")]
    Do,
    #[token("else")]
    Else,
    #[token("elseif")]
    ElseIf,
    #[token("end")]
    End,
    #[token("false")]
    False,
    #[token("for")]
    For,
    #[token("function")]
    Function,
    #[token("goto")]
    Goto,
    #[token("if")]
    If,
    #[token("in")]
    In,
    #[token("local")]
    Local,
    #[token("nil")]
    Nil,
    #[token("not")]
    Not,
    #[token("or")]
    Or,
    #[token("repeat")]
    Repeat,
    #[token("return")]
    Return,
    #[token("then")]
    Then,
    #[token("true")]
    True,
    #[token("until")]
    Until,
    #[token("while")]
    While,

    // Multi-character operators
    #[token("...")]
    VarArgs,
    #[token("..")]
    Concat,
    #[token("::")]
    LabelDelim,
    #[token("==")]
    Equal,
    #[token("~=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,
    #[token("//")]
    FloorDivide,

    // Single character operators
    #[token("=")]
    Assign,
    #[token("~")]
    BitXor,
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulus,
    #[token("^")]
    Power,
    #[token("#")]
    Length,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    #[regex(r"<[a-zA-Z_][a-zA-Z0-9_]*>", |lex| {
        lex.slice()[1..lex.slice().len()-1].to_string()
    })]
    Attribute(String),

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Floats
    #[regex(
        r"[0-9](?:_[0-9]|[0-9])*\.[0-9](?:_[0-9]|[0-9])*(?:[eE][+-]?[0-9](?:_[0-9]|[0-9])*)?",
        |lex| lex.slice().replace("_", "").parse().ok()
    )]
    #[regex(
        r"[0-9](?:_[0-9]|[0-9])*(?:[eE][+-]?[0-9](?:_[0-9]|[0-9])*)",
        |lex| lex.slice().replace("_", "").parse().ok()
    )]
    Number(f64),

    // Integers
    #[regex(r"0[xX][0-9a-fA-F]+", |lex| {
        let s = lex.slice();
        i64::from_str_radix(&s[2..], 16).ok()
    })]
    #[regex(r"[0-9](?:_[0-9]|[0-9])*", |lex| lex.slice().replace("_", "").parse().ok())]
    Integer(i64),

    // Strings
    #[regex(r"\[\[([^\]]|\][^\]])*\]\]", |lex| {
        let s = lex.slice();
        Some(s[2..s.len()-2].to_string())
    })]
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        if s.len() < 2 { return None }
        Some(unescape(&s[1..s.len()-1]))
    })]
    #[regex(r"'([^'\\]|\\.)*'", |lex| {
        let s = lex.slice();
        if s.len() < 2 { return None }
        Some(unescape(&s[1..s.len()-1]))
    })]
    StringLiteral(String),

    // Comments
    #[regex(r"--[^\n]*", logos::skip)]
    LineComment,

    #[regex(r"--\[\[([^\]]|\][^\]])*\]\]", logos::skip)]
    BlockComment,

    Eof,
}

pub struct Lexer<'source> {
    tokenizer: LogosLexer<'source, Token>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Lexer<'source> {
        Lexer {
            tokenizer: Token::lexer(source),
        }
    }

    pub fn collect(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = self.tokenizer.by_ref().filter_map(|res| res.ok()).collect();
        tokens.push(Token::Eof);
        tokens
    }
}
