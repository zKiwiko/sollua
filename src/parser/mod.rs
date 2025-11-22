/*
LUA LANGUAGE REFERENCE

chunk ::= block

    block ::= {stat} [retstat]

    stat ::=  ‘;’ |
         varlist ‘=’ explist |
         functioncall |
         label |
         break |
         goto Name |
         do block end |
         while exp do block end |
         repeat block until exp |
         if exp then block {elseif exp then block} [else block] end |
         for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
         for namelist in explist do block end |
         function funcname funcbody |
         local function Name funcbody |
         local attnamelist [‘=’ explist]

    attnamelist ::=  Name attrib {‘,’ Name attrib}

    attrib ::= [‘<’ Name ‘>’]

    retstat ::= return [explist] [‘;’]

    label ::= ‘::’ Name ‘::’

    funcname ::= Name {‘.’ Name} [‘:’ Name]

    varlist ::= var {‘,’ var}

    var ::=  Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name

    namelist ::= Name {‘,’ Name}

    explist ::= exp {‘,’ exp}

    exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef |
         prefixexp | tableconstructor | exp binop exp | unop exp

    prefixexp ::= var | functioncall | ‘(’ exp ‘)’

    functioncall ::=  prefixexp args | prefixexp ‘:’ Name args

    args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString

    functiondef ::= function funcbody

    funcbody ::= ‘(’ [parlist] ‘)’ block end

    parlist ::= namelist [‘,’ ‘...’] | ‘...’

    tableconstructor ::= ‘{’ [fieldlist] ‘}’

    fieldlist ::= field {fieldsep field} [fieldsep]

    field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp

    fieldsep ::= ‘,’ | ‘;’

    binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |
         ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ |
         ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
         and | or

    unop ::= ‘-’ | not | ‘#’ | ‘~’
*/

mod expressions;
mod helpers;
mod statements;

use crate::ast::*;
use crate::lexer::Token;

pub struct Parser<'src> {
    tokens: std::iter::Peekable<std::slice::Iter<'src, Token<'src>>>,
    _source: &'src str,
    pub errors: Vec<String>,
    pub ast: Vec<ASTNode<'src>>,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str, tokens: &'src [Token<'src>]) -> Self {
        let mut parser = Parser {
            tokens: tokens.iter().peekable(),
            _source: source,
            errors: Vec::new(),
            ast: Vec::new(),
        };
        parser.ast.reserve(tokens.len() / 4 + 16);
        parser
    }

    pub fn parse(&mut self) -> &Vec<ASTNode<'src>> {
        while let Some(token) = self.peek().cloned() {
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
                Token::Identifier(name) => {
                    self.next();
                    if let Some(stmt) = self.parse_identifier_statement(name) {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::If => {
                    self.next();
                    if let Some(stmt) = self.parse_if_statement() {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::While => {
                    self.next();
                    if let Some(stmt) = self.parse_while_statement() {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::For => {
                    self.next();
                    if let Some(stmt) = self.parse_for_statement() {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Repeat => {
                    self.next();
                    if let Some(stmt) = self.parse_repeat_statement() {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Do => {
                    self.next();
                    if let Some(stmt) = self.parse_do_block() {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Goto => {
                    self.next();
                    if let Some(stmt) = self.parse_goto_statement() {
                        self.ast.push(ASTNode::Statement(stmt));
                    }
                }
                Token::Label(name) => {
                    self.next();
                    let stmt = self.parse_label_statement(name);
                    self.ast.push(ASTNode::Statement(stmt));
                }
                Token::Semicolon => {
                    self.next();
                }
                _ => {
                    self.next();
                }
            }
        }
        &self.ast
    }
}
