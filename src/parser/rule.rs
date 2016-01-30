use std::ops::Deref;

use super::super::thin;

#[derive(Copy, Clone, Debug)]
pub struct Rule(thin::Rule);

impl Deref for Rule {
    type Target = thin::Rule;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<thin::Rule> for Rule {
    fn from(other: thin::Rule) -> Self {
        Rule(other)
    }
}

impl From<i64> for Rule {
    fn from(other: i64) -> Self {
        Rule(other as i32)
    }
}
