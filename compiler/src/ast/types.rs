use std::{fmt::{Debug, Display}, hash::Hash, ops::{Deref, DerefMut}};

use super::Expression;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Loc {
    // TODO: input file UUID or something referencing a global hashtable
    pub left: usize, pub right: usize
}

macro_rules! traits {
    ($tpe:ident) => {
        impl<T> Deref for $tpe<T> {
            type Target = T;
        
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }
        
        impl<T> DerefMut for $tpe<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.value
            }
        }
        
        impl<T : Debug> Debug for $tpe<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.value.fmt(f)
            }
        }
        
        impl<T : Clone> Clone for $tpe<T> {
            fn clone(&self) -> Self {
                Self { value: self.value.clone(), loc: self.loc.clone() }
            }
        }
        
        impl<T : Copy> Copy for $tpe<T> {}
        
        impl<T : Hash> Hash for $tpe<T> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.value.hash(state);
                self.loc.hash(state);
            }
        }
        
        impl<T : PartialEq> PartialEq for $tpe<T> {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value && self.loc == other.loc
            }
        }
        
        impl<T : Eq> Eq for $tpe<T> {}
        
        impl<T : Display> Display for $tpe<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.value.fmt(f)
            }
        }
    };
}

pub struct Tag<T> {
    pub value: T,
    pub loc: Loc
}

traits!(Tag);

pub struct OpTag<T> {
    pub value: T,
    pub loc: Option<Loc>
}

traits!(OpTag);

pub type BTag<T> = Box<OpTag<T>>;
pub type BTExpression = BTag<Expression>;

#[derive(Clone)]
pub struct QualifiedName(pub Vec<OpTag<String>>);

impl Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, item) in self.0.iter().enumerate() {
            write!(f, "{}", *item)?;
            if idx != self.0.len() - 1 {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

impl Debug for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
