use std::fmt::{Debug, Display};
use thin::Symbol;

pub trait Token: From<(Symbol, i32)> + Display + Debug {
    fn sym(&self) -> Symbol;
    fn value(&self) -> i32;
}
