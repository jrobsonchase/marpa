use libmarpa_sys::*;

use crate::thin::earley::*;
use crate::thin::grammar::Grammar;
use crate::thin::recognizer as r;
use crate::thin::recognizer::Recognizer;
use crate::thin::order::Order;
use crate::result::*;

pub struct Bocage {
    internal: Marpa_Bocage,
    // we need to keep a reference to this accessible
    // in order to read error codes.
    grammar: Grammar,
}

impl Clone for Bocage {
    fn clone(&self) -> Bocage {
        unsafe { marpa_b_ref(self.internal) };
        Bocage {
            internal: self.internal,
            grammar: self.grammar.clone(),
        }
    }
}

impl Drop for Bocage {
    fn drop(&mut self) {
        unsafe {
            marpa_b_unref(self.internal);
        }
    }
}

impl Bocage {
    pub fn internal(&self) -> Marpa_Bocage {
        self.internal
    }

    pub fn grammar(&self) -> Grammar {
        self.grammar.clone()
    }

    pub fn new_at_set(r: &Recognizer, set: EarleySet) -> Result<Bocage> {
        let r_internal = r::internal(r);
        let grammar = r::grammar(r);
        match unsafe { marpa_b_new(r_internal, set) } {
            n if n.is_null() => grammar.error_or("error creating bocage"),
            b => Ok(Bocage { internal: b, grammar }),
        }
    }

    pub fn new(r: &Recognizer) -> Result<Bocage> {
        Bocage::new_at_set(r, -1)
    }

    pub fn ambiguity_metric(&self) -> Result<i32> {
        match unsafe { marpa_b_ambiguity_metric(self.internal) } {
            i if i > 0 => Ok(i),
            -2 => self.grammar.error_or("error getting ambiguity"),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn is_null(&self) -> Result<bool> {
        match unsafe { marpa_b_is_null(self.internal) } {
            i if i >= 1 => Ok(true),
            0 => Ok(false),
            -2 => self.grammar.error_or("error getting bocage is_null"),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn top_or_node(&self) -> Result<i32> {
        match unsafe { _marpa_b_top_or_node(self.internal) } {
            i if i > 0 => Ok(i),
            code => Err(format!("failed to get top node in Bocage: {}", code).into())
        }
    }

    pub fn or_node_irl(&self, node_id: i32) -> Result<i32> {
        match unsafe { _marpa_b_or_node_irl(self.internal, node_id) } {
            i if i > 0 => Ok(i),
            code => Err(format!("failed to get or node irl in Bocage: {}", code).into())
        }
    }

    pub fn and_node_cause(&self, node_id:i32) -> Result<i32> {
        // The whole ID of NID is the external rule id of an or-node, or -1
        // if the NID is for a token and-node.
        match unsafe { _marpa_b_and_node_cause(self.internal, node_id) } {
            i if i > -2 => Ok(i),
            code => Err(format!("failed to get and_node (id {}) cause in Bocage: {}", node_id, code).into())
        }
    }

    pub fn and_node_symbol(&self, node_id:i32) -> Result<i32> {
        match unsafe { _marpa_b_and_node_symbol(self.internal, node_id) } {
            i if i > -2 => Ok(i),
            code => Err(format!("failed to get and_node symbol in Bocage: {}", code).into())
        }
    }
    pub fn and_node_predecessor(&self, node_id:i32) -> Option<i32> {
        match unsafe { _marpa_b_and_node_predecessor(self.internal, node_id) } {
            i if i > -2 => Some(i),
            _ => None
        }
    }

    pub fn get_ordering(&self) -> Option<Order> {
        match Order::new(&self) {
            Ok(o) => Some(o),
            _ => None
        }
        // TODO?
        // GIVEN_RANKING_METHOD: {
        //     my $ranking_method =
        //         $recce->[Marpa::R2::Internal::Recognizer::RANKING_METHOD];
        //     if ( $ranking_method eq 'high_rule_only' ) {
        //         do_high_rule_only($recce);
        //         last GIVEN_RANKING_METHOD;
        //     }
        //     if ( $ranking_method eq 'rule' ) {
        //         do_rank_by_rule($recce);
        //         last GIVEN_RANKING_METHOD;
        //     }
        // } ## end GIVEN_RANKING_METHOD:
    }
}

#[cfg(test)]
mod tests {

    use crate::thin::*;

    #[test]
    fn create_bocage() {
        let mut g: Grammar = Grammar::new().unwrap();
        let start = g.new_symbol().unwrap();
        g.set_start_symbol(start).unwrap();
        g.new_rule(start, &[]).unwrap();
        g.precompute().unwrap();
        assert!(g.symbol_is_nulling(start).unwrap());
        assert!(g.events().unwrap().collect::<Vec<Event>>().len() == 0);

        let mut r: Recognizer = Recognizer::new(g).unwrap();

        r.start_input().unwrap();

        let evs: Vec<Event> = r.events().unwrap().collect();
        for e in evs.iter() {
            println!("Event: {:?}", e);
        }
        assert!(evs.len() != 0);

        let _ = Bocage::new(&r).unwrap();
    }
}

result_from_borrowed!(Bocage, Recognizer);
