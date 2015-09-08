use thin::rule::Rule;
use thin::earley::EarleySet;

pub struct ProgressItem {
    pub rule: Rule,
    pub pos: i32,
    pub origin: EarleySet,
}

pub type ProgressReport = Vec<ProgressItem>;
