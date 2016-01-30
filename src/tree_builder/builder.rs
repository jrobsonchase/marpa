use thin::Rule;
use thin::Symbol;
use lexer::token::Token;
use lexer::byte_scanner::ByteToken;
use tree_builder::tree::Handle;
use tree_builder::tree::Node;
use stack::processor::Processor;
use std::collections::HashSet;

#[derive(Default)]
pub struct TreeBuilder {
    token_rules: HashSet<Rule>,
    rules: HashSet<Rule>,
}

impl TreeBuilder {
    pub fn new() -> TreeBuilder {
        Default::default()
    }

    pub fn token(&mut self, rule_id: Rule) {
        self.token_rules.insert(rule_id);
    }

    pub fn is_token(&self, rule_id: Rule) -> bool {
        self.token_rules.contains(&rule_id)
    }

    pub fn rule(&mut self, rule_id: Rule) {
        self.rules.insert(rule_id);
    }

    pub fn is_rule(&self, rule_id: Rule) -> bool {
        self.rules.contains(&rule_id)
    }
}

impl Processor for TreeBuilder {
    type Tree = Handle<ByteToken>;
    type Token = ByteToken;

    fn proc_rule(&mut self, rule: Rule, children: &[Handle<ByteToken>]) -> Handle<ByteToken> {
        Node::rule(rule, children).into()
    }

    fn proc_token(&mut self, tok: ByteToken) -> Handle<ByteToken> {
        Node::leaf(tok).into()
    }

    fn proc_null(&mut self, sym: Symbol) -> Handle<ByteToken> {
        Node::null(sym).into()
    }
}

fn rollup(children: &[Handle<ByteToken>]) -> String {
    let mut bytes = vec![];
    traverse(children, &mut bytes);
    String::from_utf8(bytes).unwrap()
}

fn traverse(handles: &[Handle<ByteToken>], out: &mut Vec<u8>) {
    for child in handles.iter() {
        match &*child.borrow() {
            &Node::Rule(_, ref chs) => {
                traverse(chs, out);
            },
            &Node::Leaf(tok) => {
                out.push(*tok);
            },
            &Node::Null(_) => {},
            &Node::Token(_, ref st) => {
                out.extend(st.as_bytes());
            }
        }
    }
}
