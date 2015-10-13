use thin::Rule;
use thin::Symbol;
use lexer::token::Token;

pub trait Engine<T> {
    fn proc_rule(&mut self, rule: Rule, children: &[T]) -> T;
    fn proc_token(&mut self, tok: Token) -> T;
    fn proc_null(&mut self, sym: Symbol) -> T;
}
