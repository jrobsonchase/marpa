use thin::Rule;
use thin::Symbol;
use lexer::token::Token;
use tree_builder::tree::Handle;
use tree_builder::tree::Node;
use stack::engine::Engine;

#[derive(Default)]
pub struct TreeBuilder;


impl TreeBuilder {
    pub fn new() -> TreeBuilder {
        Default::default()
    }
}

impl Engine<Handle> for TreeBuilder {
    fn proc_rule(&mut self, rule: Rule, children: &[Handle]) -> Handle {
        Node::tree(rule, children).into()
    }

    fn proc_token(&mut self, tok: Token) -> Handle {
        Node::leaf(tok).into()
    }

    fn proc_null(&mut self, sym: Symbol) -> Handle {
        Node::null(sym).into()
    }
}
