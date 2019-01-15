use std::ops::Range;
use thin::libmarpa_sys::*;

pub type Rule = Marpa_Rule_ID;

pub type RuleIter = Range<Marpa_Rule_ID>;
