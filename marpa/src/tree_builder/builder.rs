use crate::lexer::byte_scanner::ByteToken;
use crate::stack::processor::Processor;
use crate::thin::Rule;
use crate::thin::Symbol;
use crate::tree_builder::tree::Handle;
use crate::tree_builder::tree::Node;
use std::collections::HashSet;

#[derive(Default, Clone)]
pub struct TreeBuilder {
    token_rules: HashSet<Rule>,
    discard_rules: HashSet<Rule>,
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

    pub fn discard(&mut self, rule_id: Rule) {
        self.discard_rules.insert(rule_id);
    }

    pub fn is_discard(&mut self, rule_id: Rule) -> bool {
        self.discard_rules.contains(&rule_id)
    }
}

impl Processor for TreeBuilder {
    type Tree = Handle<ByteToken>;
    type Token = ByteToken;

    fn proc_rule(&mut self, rule: Rule, children: &[Handle<ByteToken>]) -> Handle<ByteToken> {
        if self.is_token(rule) {
            Node::token(rule, rollup_token(children)).into()
        } else if self.is_rule(rule) {
            Node::rule(rule, rollup_rule(children)).into()
        } else if self.is_discard(rule) {
            Node::Null(0).into()
        } else {
            Node::tree(rule, children).into()
        }
    }

    fn proc_token(&mut self, tok: ByteToken) -> Handle<ByteToken> {
        Node::leaf(tok).into()
    }

    fn proc_null(&mut self, sym: Symbol) -> Handle<ByteToken> {
        Node::null(sym).into()
    }
}

fn rollup_token(children: &[Handle<ByteToken>]) -> Vec<u8> {
    let mut bytes = vec![];
    rollup_token_rec(children, &mut bytes);
    bytes
}

// TODO tco
fn rollup_token_rec(handles: &[Handle<ByteToken>], out: &mut Vec<u8>) {
    for child in handles.iter() {
        match *child.borrow() {
            Node::Leaf(tok) => out.push(*tok),
            Node::Null(_) => {}
            Node::Tree(_, ref chs) => rollup_token_rec(chs, out),
            Node::Rule(_, _) => panic!("cannot rollup Rule into Token - this is an internal bug."),
            Node::Token(_, _) => panic!("cannot rollup Token into Token - this is an internal bug."),
        }
    }
}

fn rollup_rule(children: &[Handle<ByteToken>]) -> Vec<Handle<ByteToken>> {
    let mut new_children = vec![];
    rollup_rule_rec(children, &mut new_children);
    new_children
}

// TODO tco
fn rollup_rule_rec(handles: &[Handle<ByteToken>], out: &mut Vec<Handle<ByteToken>>) {
    for child in handles.iter() {
        match *child.borrow() {
            Node::Token(_, _) => out.push(child.clone()),
            Node::Rule(_, _) => out.push(child.clone()),
            Node::Null(_) => {}
            Node::Tree(_, ref chs) => rollup_rule_rec(chs, out),
            Node::Leaf(_) => {} //panic!("cannot rollup Leaf into Rule - this is an internal bug."),
        }
    }
}
