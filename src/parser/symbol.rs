use std::ops::Deref;

use super::super::thin;

#[derive(Copy, Clone, Debug)]
pub struct Symbol(thin::Symbol);

impl Deref for Symbol {
    type Target = thin::Symbol;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<thin::Symbol> for Symbol {
    fn from(other: thin::Symbol) -> Self {
        Symbol(other)
    }
}

impl From<i64> for Symbol {
    fn from(other: i64) -> Self {
        Symbol(other as i32)
    }
}

impl From<u8> for Symbol {
    fn from(other: u8) -> Self {
        Symbol(other as i32)
    }
}

impl<'a, T> From<&'a T> for Symbol where T: Into<Symbol> + Clone {
    fn from(other: &'a T) -> Symbol {
        other.clone().into()
    }
}

impl From<char> for Symbol {
    fn from(other: char) -> Self {
        (other as u8).into()
    }
}

impl Symbol {
    pub fn from_str(s: &str) -> Vec<Symbol> {
        s.as_bytes().iter().map(|b| (*b).into()).collect()
    }
}

// impl<'a, T> From<&'a [T]> for Vec<Symbol> where T: Into<Symbol> + Clone {
//     fn from(other: &'a [T]) -> Vec<Symbol> {
//         other.iter().map(|x| x.into()).collect()
//     }
// }
