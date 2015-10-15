use thin::Rule;
use thin::Symbol;
use lexer::token::Token;

pub trait Engine {
    type Tree: Clone + Default;

    fn proc_rule(&mut self, rule: Rule, children: &[Self::Tree]) -> Self::Tree;
    fn proc_token(&mut self, tok: Token) -> Self::Tree;
    fn proc_null(&mut self, sym: Symbol) -> Self::Tree;
}
