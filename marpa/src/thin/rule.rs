use libmarpa_sys::*;
use std::ops::Range;

pub type Rule = Marpa_Rule_ID;

pub type RuleIter = Range<Marpa_Rule_ID>;
