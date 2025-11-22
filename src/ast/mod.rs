use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode<'src> {
    Statement(StatementNode<'src>),
    Expression(ExpressionNode<'src>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementNode<'src> {
    Block(Vec<ASTNode<'src>>),
    Goto(&'src str),
    DoBlock {
        body: Vec<ASTNode<'src>>,
    },
    LocalAssignment {
        targets: Vec<(ExpressionNode<'src>, Option<&'src str>)>,
        values: Vec<ExpressionNode<'src>>,
    },
    Assignment {
        targets: Vec<ExpressionNode<'src>>,
        values: Vec<ExpressionNode<'src>>,
    },
    Label(&'src str),
    LocalFunctionDeclaration {
        name: &'src str,
        parameters: Vec<&'src str>,
        body: Box<ASTNode<'src>>,
    },
    FunctionDeclaration {
        name_path: Vec<&'src str>,
        is_method: bool,
        parameters: Vec<&'src str>,
        body: Box<ASTNode<'src>>,
    },
    If {
        condition: ExpressionNode<'src>,
        then_block: Vec<ASTNode<'src>>,
        else_block: Vec<ASTNode<'src>>,
    },
    While {
        condition: ExpressionNode<'src>,
        body: Vec<ASTNode<'src>>,
    },
    Repeat {
        body: Vec<ASTNode<'src>>,
        condition: ExpressionNode<'src>,
    },
    ForNumeric {
        variable: &'src str,
        start: ExpressionNode<'src>,
        end: ExpressionNode<'src>,
        step: Option<ExpressionNode<'src>>,
        body: Vec<ASTNode<'src>>,
    },
    ForGeneric {
        variables: Vec<&'src str>,
        expressions: Vec<ExpressionNode<'src>>,
        body: Vec<ASTNode<'src>>,
    },
    Return(Vec<ExpressionNode<'src>>),
    Break,
    ExpressionStatement(ExpressionNode<'src>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionNode<'src> {
    Literal(LiteralNode<'src>),
    Variable(&'src str),

    BinaryOp {
        left: Box<ExpressionNode<'src>>,
        operator: Token<'src>,
        right: Box<ExpressionNode<'src>>,
    },
    UnaryOp {
        operator: Token<'src>,
        operand: Box<ExpressionNode<'src>>,
    },
    FunctionCall {
        function: Box<ExpressionNode<'src>>,
        method: Option<&'src str>,
        arguments: Vec<ExpressionNode<'src>>,
    },
    TableConstructor {
        entries: Vec<(Option<ExpressionNode<'src>>, ExpressionNode<'src>)>,
    },
    Index {
        table: Box<ExpressionNode<'src>>,
        index: Box<ExpressionNode<'src>>,
    },
    AnonymousFunction {
        parameters: Vec<&'src str>,
        body: Box<ASTNode<'src>>,
    },
    VarArg,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralNode<'src> {
    Number(f64),
    String(&'src str),
    Boolean(bool),
    Nil,
}
