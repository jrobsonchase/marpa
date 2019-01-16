use crate::thin::earley::EarleySet;
use crate::thin::rule::Rule;

pub struct ProgressItem {
    pub rule: Rule,
    pub pos: i32,
    pub origin: EarleySet,
}

pub type ProgressReport = Vec<ProgressItem>;
