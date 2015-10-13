use thin::Rule;
use thin::Symbol;
use std::rc::Rc;
use std::cell::RefCell;
use lexer::token::Token;
use std::ops::Deref;

#[derive(Clone, Default)]
pub struct Handle(Rc<RefCell<Node>>);

pub enum Node {
    Tree(Rule, Vec<Handle>),
    Leaf(Token),
    Null(Symbol),
}

impl Node {
    pub fn leaf(tok: Token) -> Node {
        Node::Leaf(tok)
    }

    pub fn tree(rule: Rule, children: &[Handle]) -> Node {
        Node::Tree(rule, children.into())
    }

    pub fn null(sym: Symbol) -> Node {
        Node::Null(sym)
    }
}

impl Default for Node {
    fn default() -> Node {
        Node::Null(-1)
    }
}

impl Deref for Handle {
    type Target = Rc<RefCell<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Node> for Handle {
    fn from(other: Node) -> Handle {
        Handle(Rc::new(RefCell::new(other)))
    }
}
