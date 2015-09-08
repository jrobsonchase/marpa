use thin::libmarpa_sys::*;

use thin::grammar::Grammar;

use thin::recognizer::Recognizer;
use thin::recognizer as r;

use thin::earley::*;

use thin::result::*;

use std::ptr;

pub struct Bocage {
    internal: Marpa_Bocage,
    // we need to keep a reference to this accessible
    // in order to read error codes.
    grammar: Grammar,
}

pub fn internal(bocage: &Bocage) -> Marpa_Bocage {
    bocage.internal
}

pub fn grammar(bocage: &Bocage) -> Grammar {
    bocage.grammar.clone()
}

impl Clone for Bocage {
    fn clone(&self) -> Bocage {
        unsafe { marpa_b_ref(self.internal) };
        Bocage { internal: self.internal, grammar: self.grammar.clone() }
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
    pub fn new_at_set(r: Recognizer, set: EarleySet) -> Result<Bocage> {
        let r_internal = r::internal(&r);
        let grammar = r::grammar(&r);
        match unsafe { marpa_b_new(r_internal, set) } {
            n if n == ptr::null_mut() => grammar.error_or("error creating bocage"),
            b => Ok( Bocage{ internal: b, grammar: grammar }),
        }
    }

    pub fn new(r: Recognizer) -> Result<Bocage> {
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
}


#[cfg(test)]
mod tests {

    use thin::*;

    
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

        let _ = Bocage::new(r).unwrap();
    }
}
