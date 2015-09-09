use thin::libmarpa_sys::*;

use thin::{
    Grammar,
    Order,
    Result,
};

use thin::order as o;

use std::ptr;

pub struct TreeIter {
    internal: Marpa_Tree,
    // we need to keep a reference to this accessible
    // in order to read error codes.
    grammar: Grammar,
}

pub fn internal(tree: &Tree) -> Marpa_Tree {
    tree.0.internal
}

pub fn grammar(tree: &Tree) -> Grammar {
    tree.0.grammar.clone()
}

impl Clone for TreeIter {
    fn clone(&self) -> TreeIter {
        unsafe { marpa_t_ref(self.internal) };
        TreeIter { internal: self.internal, grammar: self.grammar.clone() }
    }
}

impl Drop for TreeIter {
    fn drop(&mut self) {
        unsafe { marpa_t_unref(self.internal) };
    }
}

impl TreeIter {
    pub fn new(order: Order) -> Result<TreeIter> {
        let o_internal = o::internal(&order);
        let grammar = o::grammar(&order);
        match unsafe { marpa_t_new(o_internal) } {
            n if n == ptr::null_mut() => grammar.error_or("error creating order"),
            t => Ok( TreeIter{ internal: t, grammar: grammar }),
        }
        
    }
}
// Internally, this is exactly the same as the TreeIter.
// Note that Trees returned from TreeIter's next() method could
// be modified by their iterator, so cloning/storing them is not
// recommended.
#[derive(Clone)]
pub struct Tree(TreeIter);

impl Iterator for TreeIter {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { marpa_t_next(self.internal) } {
            -1 => None,
            i if i >= 0 => Some(Tree(self.clone())),
            e => panic!("unexpected error code: {}", e),
        }
    }
}
