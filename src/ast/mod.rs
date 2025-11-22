use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode {
    Statement(StatementNode),
    Expression(ExpressionNode),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementNode {
    Block(Vec<ASTNode>),

    LocalAssignment {
        targets: Vec<ExpressionNode>,
        values: Vec<ExpressionNode>,
    },
    Assignment {
        targets: Vec<ExpressionNode>,
        values: Vec<ExpressionNode>,
    },
    LocalFunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Box<ASTNode>,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Box<ASTNode>,
    },
    If {
        condition: ExpressionNode,
        then_block: Vec<ASTNode>,
        else_block: Vec<ASTNode>,
    },
    While {
        condition: ExpressionNode,
        body: Vec<ASTNode>,
    },
    Repeat {
        body: Vec<ASTNode>,
        condition: ExpressionNode,
    },
    For {
        variable: String,
        start: ExpressionNode,
        end: ExpressionNode,
        step: Option<ExpressionNode>,
        body: Vec<ASTNode>,
    },
    Return(Vec<ExpressionNode>),
    Break,
    ExpressionStatement(ExpressionNode),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionNode {
    Literal(LiteralNode),
    Variable(String),

    BinaryOp {
        left: Box<ExpressionNode>,
        operator: Token,
        right: Box<ExpressionNode>,
    },
    UnaryOp {
        operator: Token,
        operand: Box<ExpressionNode>,
    },
    FunctionCall {
        function: Box<ExpressionNode>,
        arguments: Vec<ExpressionNode>,
    },
    TableConstructor {
        entries: Vec<(Option<ExpressionNode>, ExpressionNode)>,
    },
    Index {
        table: Box<ExpressionNode>,
        index: Box<ExpressionNode>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralNode {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
