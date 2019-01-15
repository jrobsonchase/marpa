use std::ops::Range;
use thin::libmarpa_sys::*;

pub type Symbol = Marpa_Symbol_ID;

pub type SymIter = Range<Marpa_Symbol_ID>;

pub type TokValue = i32;
