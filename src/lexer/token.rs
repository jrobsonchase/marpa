use thin::Symbol;

/// A lexed token - includes a value and type. The type refers to
/// the regexp index that matched the token while the value can be
/// used to reference an array that holds the text, number, or other
/// type of value.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Token {
    pub ty: usize,
    pub val: usize,
}


impl Token {
    pub fn new(sym: Symbol, val: i32) -> Token {
        Token{ ty: sym as usize, val: val as usize }
    }
}
