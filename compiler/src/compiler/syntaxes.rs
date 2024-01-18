use std::error::Error;

use crate::ast::{types::Loc, ParsedFile};

pub mod ast_syntax;

pub trait ParseError : Error {
    fn loc(&self) -> Loc;
}

pub trait Syntax {
    fn parse(inp: &str) -> Result<ParsedFile, Box<dyn ParseError>>;
}
