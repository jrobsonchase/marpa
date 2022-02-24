
use crate::thin::bocage::Bocage;
use crate::thin::grammar::Grammar;
use libmarpa_sys::*;

use crate::result::*;

pub struct Order {
    internal: Marpa_Order,
    // we need to keep a reference to this accessible
    // in order to read error codes.
    grammar: Grammar,
}

impl From<&Bocage> for Result<Order> {
    fn from(other: &Bocage) -> Self {
        Order::new(other)
    }
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
        Order {
            internal: self.internal,
            grammar: self.grammar.clone(),
        }
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
    pub fn new(b: &Bocage) -> Result<Order> {
        let grammar = b.grammar();
        match unsafe { marpa_o_new(b.internal()) } {
            n if n.is_null() => grammar.error_or("error creating order"),
            o => Ok(Order { internal: o, grammar }),
        }
    }

    pub fn ambiguity_metric(&self) -> Result<i32> {
        match unsafe { marpa_o_ambiguity_metric(self.internal) } {
            -2 => self.grammar.error_or("error getting order ambiguity metric"),
            m if m >= 1 => Ok(m),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn is_null(&self) -> Result<bool> {
        match unsafe { marpa_o_is_null(self.internal) } {
            -2 => self.grammar.error_or("error getting order is_null"),
            0 => Ok(false),
            1 => Ok(true),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn high_rank_only_set(&mut self, high_only: bool) -> Result<()> {
        match unsafe { marpa_o_high_rank_only_set(self.internal, high_only as i32) } {
            -2 => self.grammar.error_or("error setting high rank only"),
            0 | 1 => Ok(()),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn high_rank_only(&self) -> Result<bool> {
        match unsafe { marpa_o_high_rank_only(self.internal) } {
            -2 => self.grammar.error_or("error getting high rank only"),
            0 => Ok(false),
            1 => Ok(true),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn rank(&mut self) -> Result<()> {
        match unsafe { marpa_o_rank(self.internal) } {
            -2 => self.grammar.error_or("error ranking order"),
            i if i >= 0 => Ok(()),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn or_node_and_node_count(&mut self, or_node_id: usize) -> Result<i32> {
        match unsafe { _marpa_o_or_node_and_node_count(self.internal, or_node_id as i32) } {
            m if m >= 0 => Ok(m),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn or_node_and_node_ids(&mut self, or_node_id: usize) -> Vec<i32> {
        let mut ids = Vec::new();
        match unsafe { _marpa_o_or_node_and_node_count(self.internal, or_node_id as i32) } {
            -1 => {},
            c if c < -1 => panic!("Invalid or node ID {}", or_node_id),
            count => for ix in 0 .. count {
                    let and_node_id = unsafe {
                        _marpa_o_or_node_and_node_id_by_ix(self.internal, or_node_id as i32, ix) };
                    ids.push(and_node_id);
            }
        }
        ids
    }
}
