pub mod types;

use types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tpe {
    Name(OpTag<String>),
    // TODO: pointer, reference, array, parameterized type support
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    String(OpTag<String>),
    Numeric(OpTag<String>),
    Char(OpTag<String>),
    Boolean(OpTag<String>)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comp {
    LessThan, LessThanEq, Eq, GreaterThanEq, GreaterThan,
    NotEq
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodName {
    Normal(OpTag<String>),
    Plus, Minus, Times, Divide, Modulo, Comparison(Comp), Dereference, Reference, Ternery,
    BoolAnd, BoolOr, BoolNot, BoolXor,
    BitAnd, BitOr, BitNot, BitXor, BitShl, BitShr, BitUShr,
    Cast, Bitcast,
    ArrayIndex,
    Return,
    ExprAssign,
    // stuff like +=, -=, &&=, et cetera.
    ExprAssignOp(Box<MethodName>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(OpTag<Literal>),
    MethodCall { receiver: Option<BTExpression>, name: OpTag<String>, args: Vec<OpTag<Expression>>, type_params: Vec<OpTag<Tpe>> },
    VarAccess(OpTag<String>),
    FieldAccess { left: BTExpression, name: OpTag<String> },
    VarDef { name: OpTag<String>, explicit_type: Option<OpTag<Tpe>>, value: BTExpression }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    ExpressionEval(OpTag<Expression>),
    If { condition: OpTag<Expression>, block: Vec<OpTag<Statement>>, else_block: Option<Vec<OpTag<Statement>>> },
    Label(OpTag<String>),
    While { condition: OpTag<Expression>, block: Vec<OpTag<Statement>> }
}

pub struct File {
    package: Option<QualifiedName>,
}
