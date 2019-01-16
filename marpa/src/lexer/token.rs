use crate::thin::Symbol;
use std::fmt::{Debug, Display};

pub trait Token: From<(Symbol, i32)> + Display + Debug {
    fn sym(&self) -> Symbol;
    fn value(&self) -> i32;
}
