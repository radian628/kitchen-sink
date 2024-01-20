pub mod syntaxes;

use crate::bytecode::{IntSize, FloatSize};

// TODO: display implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Integer { signed: bool, size: IntSize },
    Float(FloatSize),
    Char(IntSize),
    Bool
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UUID(u64);

pub struct StructData {
    uuid: UUID,
    name: String
}

pub enum Type {
    Primitive(PrimitiveType),
    Struct {  },
    Parameter { name: String },
    Dynamic
}

pub enum MemoryRepr {
    InPlace,
    Dynamic
}
