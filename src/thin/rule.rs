use std::ops::Range;
use crate::thin::libmarpa_sys::*;

pub type Rule = Marpa_Rule_ID;

pub type RuleIter = Range<Marpa_Rule_ID>;
