use thin::Rule;
use thin::Symbol;
use lexer::token::Token;

pub trait Processor {
    type Token: Token;
    type Tree: Clone + Default;

    fn proc_rule(&mut self, rule: Rule, children: &[Self::Tree]) -> Self::Tree;
    fn proc_token(&mut self, tok: Self::Token) -> Self::Tree;
    fn proc_null(&mut self, sym: Symbol) -> Self::Tree;
}
