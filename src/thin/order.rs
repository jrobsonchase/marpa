use thin::libmarpa_sys::*;

use thin::grammar::Grammar;

use thin::bocage::Bocage;
use thin::bocage as b;

use thin::result::*;

use std::ptr;

pub struct Order {
    internal: Marpa_Order,
    // we need to keep a reference to this accessible
    // in order to read error codes.
    grammar: Grammar,
}

pub fn internal(order: &Order) -> Marpa_Order {
    order.internal
}

pub fn grammar(order: &Order) -> Grammar {
    order.grammar.clone()
}

impl Clone for Order {
    fn clone(&self) -> Order {
        unsafe { marpa_o_ref(self.internal) };
        Order { internal: self.internal, grammar: self.grammar.clone() }
    }
}

impl Drop for Order {
    fn drop(&mut self) {
        unsafe {
            marpa_o_unref(self.internal);
        }
    }
}

impl Order {
    pub fn new(b: Bocage) -> Result<Order> {
        let b_internal = b::internal(&b);
        let grammar = b::grammar(&b);
        match unsafe { marpa_o_new(b_internal) } {
            n if n == ptr::null_mut() => grammar.error_or("error creating order"),
            o => Ok( Order{ internal: o, grammar: grammar }),
        }
    }


    //TODO: the rest of these
}
