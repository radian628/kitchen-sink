use crate::ast::{types::{OpTag, Loc, QualifiedName}, MethodName, Expression, Statement, Tpe, FunctionDef, Declaration, ParsedFile};

use super::Syntax;

peg::parser! {
    grammar ast() for str {
        // stole this from uwulang, can't remember if it works
        rule ___ = [' ' | '\n']*
        rule __ = "//" [^'\n']*
        rule _ = ___ __? ___

        rule tag<T>(inner: rule<T>) -> OpTag<T> =
            left:position!() item:inner() right:position!() { OpTag { value: item, loc: Some(Loc { left, right }) } }
        
        rule ident() -> OpTag<String> = tag(<v:$(['a'..='z' | 'A'..='Z'] (['a'..='z' | 'A'..='Z' | '_' | '0'..='9'])*) { v.to_string() }>)
        
        rule qualified_name() -> QualifiedName =
            items:(ident() ** ".") { QualifiedName(items) }
        
        rule func_name() -> MethodName =
            "+" { MethodName::Plus } /
            "-" { MethodName::Minus } /
            "*" { MethodName::Times } /
            "/" { MethodName::Divide } /
            "%" { MethodName::Modulo } /
            // TODO: comparisons
            "*" { MethodName::Dereference } /
            "&" { MethodName::Reference } /
            // TODO: ternary
            "&&" { MethodName::BoolAnd } /
            "||" { MethodName::BoolOr } /
            "!" { MethodName::BoolNot } /
            "return" { MethodName::Return } /
            v:ident() { MethodName::Normal(v) }
            // TODO: the rest of them lol
        
        rule expression() -> OpTag<Expression> =
            "(" _
                e:tag(<
                    name:tag(<func_name()>) _ args:expression() ** _ { Expression::MethodCall { receiver: None, name, args, type_params: vec![] } } /
                    lit:tag(<v:tag(<value:$(['0'..='9']+) { value.to_string() }>) { crate::ast::Literal::Numeric(v) }>) { Expression::Literal(lit) }
                    // TODO: other kinds of expression
                    >)
            _ ")" { e }
        
        rule statement() -> OpTag<Statement> = tag(<
            e:expression() { Statement::ExpressionEval(e) }
            // TODO: other kinds of statement
        >)

        rule tpe() -> OpTag<Tpe> = tag(<
            name:ident() { Tpe::Name(name) }
        >)

        rule function_def() -> FunctionDef =
            "fun" _ name:ident() _ return_tpe:("->" _ t:tpe() { t })? _ "(" _ parameters:(tpe:tpe() _ name:ident() { (name, tpe) }) ** ("," _) _ ")" _ "{" _ block:statement()* _ "}" { FunctionDef { name, parameters, return_tpe, block } }
        
        rule decl() -> Declaration =
            func:function_def() { Declaration::Func(func) }
            // TODO: type declarations
        
        pub rule file() -> ParsedFile =
            // TODO: package, imports
            _ decls:decl() ** _ _ { ParsedFile { package: None, imports: vec![], decls } }
    }
}

pub struct AstSyntax;

impl Syntax for AstSyntax {
    fn parse(inp: &str) -> Result<ParsedFile, Box<dyn super::ParseError>> {
        // TODO: defuck
        Ok(ast::file(inp).unwrap())
    }
}
