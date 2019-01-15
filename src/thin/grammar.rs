use thin::libmarpa_sys::*;

use result::*;

use thin::{Config, EventIter, Rule, RuleIter, SymIter, Symbol};

use std::ptr;

pub struct Grammar {
    internal: Marpa_Grammar,
}

pub fn internal(grammar: &Grammar) -> Marpa_Grammar {
    grammar.internal
}

impl Clone for Grammar {
    fn clone(&self) -> Grammar {
        unsafe { marpa_g_ref(self.internal) };
        Grammar { internal: self.internal }
    }
}

impl Drop for Grammar {
    fn drop(&mut self) {
        unsafe {
            marpa_g_unref(self.internal);
        }
    }
}

impl Grammar {
    pub fn new() -> Result<Self> {
        let mut cfg = Config::new();
        unsafe {
            let c_grammar = marpa_g_new(&mut cfg.internal());

            try!(cfg.error());

            assert!(marpa_g_force_valued(c_grammar) >= 0);
            Ok(Grammar { internal: c_grammar })
        }
    }

    pub fn with_config(cfg: Config) -> Result<Grammar> {
        let mut cfg = cfg;
        unsafe {
            let c_grammar = marpa_g_new(&mut cfg.internal());

            try!(cfg.error());

            assert!(marpa_g_force_valued(c_grammar) >= 0);
            Ok(Grammar { internal: c_grammar })
        }
    }

    // either return the error result from the grammar or an empty Ok
    fn error(&self) -> Result<()> {
        match unsafe { marpa_g_error(self.internal, ptr::null_mut()) } {
            0 => Ok(()),
            code => {
                unsafe { marpa_g_error_clear(self.internal) };
                err_code(code)
            }
        }
    }

    // either gets the error code from the grammar, or, in the event that there
    // is no error code, provide an error from a string.
    pub fn error_or<T>(&self, s: &str) -> Result<T> {
        match self.error() {
            Ok(()) => err(s),
            Err(error) => Err(error),
        }
    }

    pub fn new_symbol(&mut self) -> Result<Symbol> {
        match unsafe { marpa_g_symbol_new(self.internal) } {
            -2 => self.error_or("error creating new symbol"),
            sym => Ok(sym),
        }
    }

    pub fn get_start_symbol(&self) -> Result<Symbol> {
        match unsafe { marpa_g_start_symbol(self.internal) } {
            -1 => err_nosym(),
            -2 => self.error_or("error getting start symbol"),
            sym_id => Ok(sym_id),
        }
    }

    pub fn set_start_symbol(&mut self, sym: Symbol) -> Result<Symbol> {
        match unsafe { marpa_g_start_symbol_set(self.internal, sym) } {
            -1 => err_nosym(),
            -2 => self.error_or("error setting start symbol"),
            sym_id => Ok(sym_id),
        }
    }

    pub fn num_symbols(&self) -> Result<i32> {
        match unsafe { marpa_g_highest_symbol_id(self.internal) } {
            -2 => self.error_or("error getting highest symbol"),
            max => Ok(max + 1),
        }
    }

    pub fn symbols(&self) -> Result<SymIter> {
        Ok(0..try!(self.num_symbols()))
    }

    pub fn symbol_is_accessible(&self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_accessible(self.internal, sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => self.error_or("error checking symbol accessibility"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_is_nullable(&self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_nullable(self.internal, sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => self.error_or("error checking symbol nullability"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_is_nulling(&self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_nulling(self.internal, sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => self.error_or("error checking symbol nullness"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_is_productive(&self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_productive(self.internal, sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => self.error_or("error checking symbol productivity"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_is_terminal(&self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_terminal(self.internal, sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => self.error_or("error checking symbol terminality"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_set_terminal(&mut self, sym: Symbol, term: bool) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_terminal_set(self.internal, sym, term as i32) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => self.error_or("error setting symbol terminality"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn new_rule(&mut self, lhs: Symbol, rhs: &[Symbol]) -> Result<Rule> {
        let rhs_ptr = rhs.as_ptr();
        let rhs_len = rhs.len() as i32;
        match unsafe { marpa_g_rule_new(self.internal, lhs, rhs_ptr as *mut i32, rhs_len) } {
            -2 => self.error_or("error creating new rule"),
            rule => Ok(rule),
        }
    }

    pub fn rules(&self) -> Result<RuleIter> {
        let max = unsafe { marpa_g_highest_rule_id(self.internal) };
        match max {
            -2 => self.error_or("error getting highest symbol"),
            max => Ok(0..max + 1),
        }
    }

    pub fn rule_is_accessible(&self, rule: Rule) -> Result<bool> {
        match unsafe { marpa_g_rule_is_accessible(self.internal, rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule accessbility"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_is_nullable(&self, rule: Rule) -> Result<bool> {
        match unsafe { marpa_g_rule_is_nullable(self.internal, rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule nullability"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_is_nulling(&self, rule: Rule) -> Result<bool> {
        match unsafe { marpa_g_rule_is_nulling(self.internal, rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule nullness"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_is_loop(&self, rule: Rule) -> Result<bool> {
        match unsafe { marpa_g_rule_is_loop(self.internal, rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule loop"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_is_productive(&self, rule: Rule) -> Result<bool> {
        match unsafe { marpa_g_rule_is_productive(self.internal, rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule productivity"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_lhs(&self, rule: Rule) -> Result<Symbol> {
        match unsafe { marpa_g_rule_lhs(self.internal, rule) } {
            -1 => err_norule(),
            -2 => self.error_or("error getting rule lhs"),
            symid => Ok(symid),
        }
    }

    pub fn rule_length(&self, rule: Rule) -> Result<i32> {
        match unsafe { marpa_g_rule_length(self.internal, rule) } {
            -2 => self.error_or("error getting rule length"),
            len => Ok(len),
        }
    }

    pub fn rule_rhs_ix(&self, rule: Rule, ix: i32) -> Result<Symbol> {
        match unsafe { marpa_g_rule_rhs(self.internal, rule, ix) } {
            -1 => err_norule(),
            -2 => self.error_or("error getting rhs ix"),
            symid => Ok(symid),
        }
    }

    pub fn rule_rhs(&self, rule: Rule) -> Result<Vec<Symbol>> {
        let len = try!(self.rule_length(rule));
        let mut syms: Vec<Symbol> = vec![];
        for id in 0..len {
            let sym = try!(self.rule_rhs_ix(rule, id));

            syms.push(sym);
        }
        Ok(syms)
    }

    pub fn rule_is_proper_separation(&self, rule: Rule) -> Result<bool> {
        match unsafe { marpa_g_rule_is_proper_separation(self.internal, rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule separation"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn sequence_min(&self, rule: Rule) -> Result<i32> {
        match unsafe { marpa_g_sequence_min(self.internal, rule) } {
            -1 => err_notaseq(),
            -2 => self.error_or("error getting sequence min"),
            i => Ok(i),
        }
    }

    pub fn rule_is_sequence(&self, rule: Rule) -> Result<bool> {
        match unsafe { marpa_g_sequence_min(self.internal, rule) } {
            -1 => Ok(false),
            -2 => self.error_or("error getting if sequence"),
            _ => Ok(true),
        }
    }

    pub fn new_sequence(&self, lhs: Symbol, rhs: Symbol, sep: Symbol, nonempty: bool, proper: bool) -> Result<Rule> {
        match unsafe {
            marpa_g_sequence_new(
                self.internal,
                lhs,
                rhs,
                sep,
                nonempty as i32,
                if proper { MARPA_PROPER_SEPARATION } else { 0 },
            )
        } {
            -2 => self.error_or("error creating sequence"),
            ruleid => Ok(ruleid),
        }
    }

    pub fn sequence_separator(&self, rule: Rule) -> Result<Rule> {
        match unsafe { marpa_g_sequence_separator(self.internal, rule) } {
            -1 => self.error_or("Rule has no separator"),
            -2 => self.error_or("error getting sequence separator"),
            ruleid => Ok(ruleid),
        }
    }

    pub fn symbol_is_counted(&self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_counted(self.internal, sym) } {
            -1 => err_nosym(),
            -2 => self.error_or("error getting if symbol is counted"),
            0 => Ok(false),
            1 => Ok(true),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_rank_set(&mut self, rule: Rule, rank: i32) -> Result<()> {
        unsafe { marpa_g_rule_rank_set(self.internal, rule, rank) };
        self.error()
    }

    pub fn rule_rank_get(&self, rule: Rule) -> Result<i32> {
        let rank = unsafe { marpa_g_rule_rank(self.internal, rule) };
        if rank == -2 {
            match self.error() {
                Ok(()) => Ok(-2),
                Err(err) => Err(err),
            }
        } else {
            Ok(rank)
        }
    }

    pub fn rule_null_high_set(&mut self, rule: Rule, high: bool) -> Result<()> {
        match unsafe { marpa_g_rule_null_high_set(self.internal, rule, high as i32) } {
            -1 => err_norule(),
            -2 => self.error_or("error setting null high"),
            c if c == 0 || c == 1 => Ok(()),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_null_high(&mut self, rule: Rule) -> Result<bool> {
        match unsafe { marpa_g_rule_null_high(self.internal, rule) } {
            -1 => err_norule(),
            -2 => self.error_or("error setting null high"),
            0 => Ok(false),
            1 => Ok(true),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn completion_symbol_activate(&mut self, sym: Symbol, reactivate: bool) -> Result<()> {
        match unsafe { marpa_g_completion_symbol_activate(self.internal, sym, reactivate as i32) } {
            -2 => self.error_or("error setting symbol to reactivate"),
            _ => Ok(()),
        }
    }

    pub fn nulled_symbol_activate(&mut self, sym: Symbol, reactivate: bool) -> Result<()> {
        match unsafe { marpa_g_nulled_symbol_activate(self.internal, sym, reactivate as i32) } {
            -2 => self.error_or("error setting symbol to reactivate"),
            _ => Ok(()),
        }
    }

    pub fn prediction_symbol_activate(&mut self, sym: Symbol, reactivate: bool) -> Result<()> {
        match unsafe { marpa_g_prediction_symbol_activate(self.internal, sym, reactivate as i32) } {
            -2 => self.error_or("error setting symbol to reactivate"),
            _ => Ok(()),
        }
    }

    pub fn symbol_is_completion_event(&mut self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_completion_event(self.internal, sym) } {
            -1 => err_nosym(),
            -2 => self.error_or("error getting completion event"),
            0 => Ok(false),
            1 => Ok(true),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn symbol_is_completion_event_set(&mut self, sym: Symbol, val: bool) -> Result<()> {
        match unsafe { marpa_g_symbol_is_completion_event_set(self.internal, sym, val as i32) } {
            -1 => err_nosym(),
            -2 => self.error_or("error setting completion event"),
            0 => Ok(()),
            1 => Ok(()),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn symbol_is_nulled_event(&mut self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_nulled_event(self.internal, sym) } {
            -1 => err_nosym(),
            -2 => self.error_or("error getting nulled event"),
            0 => Ok(false),
            1 => Ok(true),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn symbol_is_nulled_event_set(&mut self, sym: Symbol, val: bool) -> Result<()> {
        match unsafe { marpa_g_symbol_is_nulled_event_set(self.internal, sym, val as i32) } {
            -1 => err_nosym(),
            -2 => self.error_or("error setting nulled event"),
            0 => Ok(()),
            1 => Ok(()),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn symbol_is_prediction_event(&mut self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_g_symbol_is_prediction_event(self.internal, sym) } {
            -1 => err_nosym(),
            -2 => self.error_or("error getting prediction event"),
            0 => Ok(false),
            1 => Ok(true),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn symbol_is_prediction_event_set(&mut self, sym: Symbol, val: bool) -> Result<()> {
        match unsafe { marpa_g_symbol_is_prediction_event_set(self.internal, sym, val as i32) } {
            -1 => err_nosym(),
            -2 => self.error_or("error setting prediction event"),
            0 => Ok(()),
            1 => Ok(()),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn events(&self) -> Result<EventIter> {
        unsafe {
            match marpa_g_event_count(self.internal) {
                -2 => self.error_or("error getting event count"),
                count => Ok(EventIter::new(count, self.clone())),
            }
        }
    }

    pub fn precompute(&mut self) -> Result<()> {
        match unsafe { marpa_g_precompute(self.internal) } {
            -2 => self.error_or("error precomputing grammar"),
            i if i >= 0 => Ok(()),
            i => panic!("unexpected error code: {}", i),
        }
    }

    pub fn is_precomputed(&self) -> Result<bool> {
        match unsafe { marpa_g_is_precomputed(self.internal) } {
            -2 => self.error_or("error getting is_precomputed"),
            0 => Ok(false),
            1 => Ok(true),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn has_cycle(&self) -> Result<bool> {
        match unsafe { marpa_g_has_cycle(self.internal) } {
            -2 => self.error_or("error getting has_cycle"),
            0 => Ok(false),
            1 => Ok(true),
            err => panic!("unexpected error code: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use thin::event::Event;
    use thin::grammar::Grammar;
    use thin::rule::Rule;
    use thin::symbol::Symbol;

    fn new_grammar() -> Grammar {
        Grammar::new().unwrap()
    }

    #[test]
    fn create_grammar() {
        new_grammar();
    }

    #[test]
    fn iter_syms() {
        let mut g = new_grammar();
        for _ in 0..5 {
            g.new_symbol().unwrap();
        }
        let ids: Vec<i32> = vec![0, 1, 2, 3, 4];
        assert!(g.symbols().unwrap().collect::<Vec<Symbol>>() == ids);
    }

    #[test]
    fn iter_rules() {
        let mut g = new_grammar();
        for _ in 0..6 {
            g.new_symbol().unwrap();
        }

        g.new_rule(0.into(), &[1, 2]).unwrap();
        g.new_rule(1.into(), &[3, 4]).unwrap();
        g.new_rule(2.into(), &[]).unwrap();

        let ids: Vec<i32> = vec![0, 1, 2];

        assert!(g.rules().unwrap().collect::<Vec<Rule>>() == ids);
        assert!(g.rule_is_accessible(0).unwrap());
    }

    #[test]
    fn set_terminal() {
        let mut g = Grammar::new().unwrap();
        let s = g.new_symbol().unwrap();
        assert!(g.symbol_is_terminal(s).unwrap() == false);
        g.symbol_set_terminal(s, true).unwrap();
        assert!(g.symbol_is_terminal(s).unwrap() == true);
        let term = g.symbol_set_terminal(s, false);
        match term {
            Ok(_) => assert!(false),
            _ => {}
        }
    }

    #[test]
    fn set_start() {
        let mut g: Grammar = Grammar::new().unwrap();

        let sym: Symbol = g.new_symbol().unwrap();
        assert!(g.set_start_symbol(sym).unwrap() == sym);

        assert!(g.get_start_symbol().unwrap() == sym);
    }

    #[test]
    fn precompute() {
        let mut g: Grammar = new_grammar();
        let start = g.new_symbol().unwrap();
        g.set_start_symbol(start).unwrap();
        g.new_rule(start, &[]).unwrap();
        g.precompute().unwrap();
        assert!(g.symbol_is_nulling(start).unwrap());
        assert!(g.events().unwrap().collect::<Vec<Event>>().len() == 0);
    }
}

impl From<Config> for Result<Grammar> {
    fn from(other: Config) -> Self {
        Grammar::with_config(other)
    }
}
