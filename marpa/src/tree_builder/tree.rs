use crate::lexer::token::Token;
use crate::thin::Rule;
use crate::thin::Symbol;
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Default, Debug)]
pub struct Handle<T: Token>(Rc<RefCell<Node<T>>>);

#[derive(Debug)]
pub enum Node<T>
where
    T: Token,
{
    Rule(Rule, Vec<Handle<T>>),
    Tree(Rule, Vec<Handle<T>>),
    Token(Rule, Vec<u8>),
    Leaf(T),
    Null(Symbol),
}

impl<T: Token> fmt::Display for Node<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Tree(ref rule, ref children) => {
                write!(f, "Tree({},", rule)?;
                for child in children {
                    write!(f, " {}", child)?;
                }
                write!(f, ")")?;
            }
            Node::Rule(ref rule, ref children) => {
                write!(f, "Rule({},", rule)?;
                for child in children {
                    write!(f, " {}", child)?;
                }
                write!(f, ")")?;
            }
            Node::Token(ty, ref val) => {
                write!(f, "Token({}, ", ty)?;
                match ::std::str::from_utf8(&val) {
                    Ok(s) => write!(f, "\"{}\"", s)?,
                    Err(_) => write!(f, "{:?}", val)?,
                }
                write!(f, ")")?;
            }
            Node::Leaf(ref tok) => {
                write!(f, "Leaf({})", tok)?;
            }
            Node::Null(sym) => {
                write!(f, "Null({})", sym)?;
            }
        }
        Ok(())
    }
}

impl<T: Token> fmt::Display for Handle<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self.borrow())
    }
}

impl<T: Token> Node<T> {
    pub fn token<V: Into<Vec<u8>>>(tok: Symbol, val: V) -> Node<T> {
        Node::Token(tok, val.into())
    }

    pub fn leaf(tok: T) -> Node<T> {
        Node::Leaf(tok)
    }

    pub fn rule<V: Into<Vec<Handle<T>>>>(rule: Rule, children: V) -> Node<T>
    where
        T: Clone,
    {
        Node::Rule(rule, children.into())
    }

    pub fn tree<V: Into<Vec<Handle<T>>>>(rule: Rule, children: V) -> Node<T>
    where
        T: Clone,
    {
        Node::Tree(rule, children.into())
    }

    pub fn null(sym: Symbol) -> Node<T> {
        Node::Null(sym)
    }
}

impl<T: Token> Default for Node<T>
where
    T: Default,
{
    fn default() -> Node<T> {
        Node::Null(-1)
    }
}

impl<T: Token> Deref for Handle<T> {
    type Target = Rc<RefCell<Node<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Token> From<Node<T>> for Handle<T> {
    fn from(other: Node<T>) -> Handle<T> {
        Handle(Rc::new(RefCell::new(other)))
    }
}
