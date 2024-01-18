pub mod types;

use types::*;

use crate::bytecode::{IntSize, FloatSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Integer { signed: bool, size: IntSize },
    Float(FloatSize),
    Char(IntSize),
    Bool
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tpe {
    Name(OpTag<String>),
    // TODO: reference, array, parameterized type support
    Pointer(BTag<Tpe>),
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
    MethodCall { receiver: Option<BTExpression>, name: OpTag<MethodName>, args: Vec<OpTag<Expression>>, type_params: Vec<OpTag<Tpe>> },
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

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: OpTag<String>,
    // TODO: type parameters
    pub parameters: Vec<(OpTag<String>, OpTag<Tpe>)>,
    pub return_tpe: Option<OpTag<Tpe>>,
    pub block: Vec<OpTag<Statement>>
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Func(FunctionDef),
    // TODO: type definition struct
}

#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: QualifiedName,
    pub alias: Option<String>
}

#[derive(Debug, Clone)]
pub struct ParsedFile {
    pub package: Option<QualifiedName>,
    pub imports: Vec<ImportStatement>,
    pub decls: Vec<Declaration>
}
