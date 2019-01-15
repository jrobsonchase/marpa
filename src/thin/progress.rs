use thin::earley::EarleySet;
use thin::rule::Rule;

pub struct ProgressItem {
    pub rule: Rule,
    pub pos: i32,
    pub origin: EarleySet,
}

pub type ProgressReport = Vec<ProgressItem>;
