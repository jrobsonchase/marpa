use crate::thin::order as o;
use crate::thin::{Grammar, Order, Result, Value};
use libmarpa_sys::*;

pub struct Tree {
    internal: Marpa_Tree,
    // we need to keep a reference to this accessible
    // in order to read error codes.
    grammar: Grammar,
}

result_from!(Tree, Order);

pub fn internal(tree: &Tree) -> Marpa_Tree {
    tree.internal
}

pub fn grammar(tree: &Tree) -> Grammar {
    tree.grammar.clone()
}

impl Clone for Tree {
    fn clone(&self) -> Tree {
        unsafe { marpa_t_ref(self.internal) };
        Tree {
            internal: self.internal,
            grammar: self.grammar.clone(),
        }
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        unsafe { marpa_t_unref(self.internal) };
    }
}

impl Tree {
    pub fn new(order: Order) -> Result<Tree> {
        let o_internal = o::internal(&order);
        let grammar = o::grammar(&order);
        match unsafe { marpa_t_new(o_internal) } {
            n if n.is_null() => grammar.error_or("error creating order"),
            t => Ok(Tree { internal: t, grammar }),
        }
    }
}

impl Iterator for Tree {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { marpa_t_next(self.internal) } {
            -1 => None,
            i if i >= 0 => {
                if let Ok(val) = Value::new(&self) {
                    Some(val)
                } else {
                    None
                }
            }
            e => panic!("unexpected error code: {}", e),
        }
    }
}
