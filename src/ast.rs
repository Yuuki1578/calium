pub trait ASTSpec {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Remainder,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOperator {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
pub enum NodeExpr {
    IntExpr(i64),
    UnaryExpr {
        unary_op: UnaryOperator,
        next: Box<NodeExpr>,
    },
    BinaryExpr {
        binary_op: BinaryOperator,
        left_node: Box<NodeExpr>,
        right_node: Box<NodeExpr>,
    },
}

impl ASTSpec for BinaryOperator {}
impl ASTSpec for UnaryOperator {}
impl ASTSpec for NodeExpr {}
