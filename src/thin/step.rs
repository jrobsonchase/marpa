use thin::{Rule, Symbol, TokValue};

#[derive(Debug)]
pub enum Step {
    Rule(Rule, i32, i32),
    Token(Symbol, i32, TokValue),
    NullingSymbol(Symbol, i32),
    Inactive,
    Initial,
}

impl Step {
    pub fn rule(ruleid: Rule, loc: i32, last: i32) -> Step {
        Step::Rule(ruleid, loc, last)
    }

    pub fn token(symid: Symbol, loc: i32, value: TokValue) -> Step {
        Step::Token(symid, loc, value)
    }

    pub fn nulling_symbol(symid: Symbol, loc: i32) -> Step {
        Step::NullingSymbol(symid, loc)
    }
}
