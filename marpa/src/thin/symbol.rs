use crate::thin::libmarpa_sys::*;
use std::ops::Range;

pub type Symbol = Marpa_Symbol_ID;

pub type SymIter = Range<Marpa_Symbol_ID>;

pub type TokValue = i32;
