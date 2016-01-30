use thin::Symbol;

pub trait Token : From<(Symbol, i32)> {
    fn sym(&self) -> Symbol;
    fn value(&self) -> i32;
}

/// A lexed token - includes a value and type. The type refers to
/// the regexp index that matched the token while the value can be
/// used to reference an array that holds the text, number, or other
/// type of value.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct ReToken {
    pub ty: Symbol,
    pub val: i32,
}


impl ReToken {
    pub fn new(sym: Symbol, val: i32) -> ReToken {
        ReToken{ ty: sym, val: val }
    }
}

impl From<(Symbol, i32)> for ReToken {
    fn from((sym, val): (Symbol, i32)) -> Self {
        ReToken::new(sym, val)
    }
}

impl Token for ReToken {
    fn sym(&self) -> Symbol {
        self.ty
    }
    fn value(&self) -> i32 {
        self.val
    }
}
