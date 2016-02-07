use thin::Symbol;
use std::fmt::{Display, Debug};

pub trait Token : From<(Symbol, i32)> + Display + Debug {
    fn sym(&self) -> Symbol;
    fn value(&self) -> i32;
}
