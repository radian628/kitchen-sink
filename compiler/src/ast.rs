#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Loc {
    // TODO: input file UUID or something referencing a global hashtable
    pub left: usize, pub right: usize
}

// TODO: impl debug, clone, copy, hash, partialeq, eq, display when T implements them
pub struct Tag<T> {
    pub value: T,
    pub loc: Loc
}

pub type BTag<T> = Box<Tag<T>>;
pub type BTExpression = BTag<Expression>;

pub enum Tpe {
    Name(Tag<String>),
    // TODO: pointer, reference, array, parameterized type support
}

pub enum Literal {
    // TODO: numeric (with type? how tf do we represent the type), char, boolean literals
    String(Tag<String>)
}

pub enum Comp {
    LessThan, LessThanEq, Eq, GreaterThanEq, GreaterThan,
    NotEq
}

pub enum MethodName {
    Normal(String),
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

pub enum Expression {
    Literal(Tag<Literal>),
    MethodCall { receiver: Option<BTExpression>, name: Tag<String>, args: Vec<Tag<Expression>>, type_params: Vec<Tag<Tpe>> },
    VarAccess(Tag<String>),
    FieldAccess { left: BTExpression, name: Tag<String> },
    VarDef { name: Tag<String>, explicit_type: Option<Tag<Tpe>>, value: BTExpression }
}


pub enum Statement {
    ExpressionEval(Tag<Expression>),
    If { condition: Tag<Expression>, block: Vec<Tag<Statement>>, else_block: Option<Vec<Tag<Statement>>> },
    Label(Tag<String>)
}
