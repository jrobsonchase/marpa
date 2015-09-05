use libmarpa_sys::{
    Marpa_Grammar,
    Marpa_Config,
    Marpa_Error_Code,
    Marpa_Symbol_ID,
    Marpa_Rule_ID,
    marpa_c_init,
    marpa_c_error,
    marpa_g_new,
    marpa_g_error,
    marpa_g_force_valued,
    marpa_g_ref,
    marpa_g_unref,
    marpa_g_symbol_new,
    marpa_g_start_symbol,
    marpa_g_start_symbol_set,
    marpa_g_highest_symbol_id,
    marpa_g_symbol_is_accessible,
    marpa_g_symbol_is_nullable,
    marpa_g_symbol_is_nulling,
    marpa_g_symbol_is_productive,
    marpa_g_symbol_is_terminal,
    marpa_g_symbol_is_terminal_set,
    marpa_g_highest_rule_id,
    marpa_g_rule_new,
    marpa_g_rule_is_accessible,
    marpa_g_rule_is_nullable,
    marpa_g_rule_is_nulling,
    marpa_g_rule_is_loop,
    marpa_g_rule_is_productive,
    marpa_g_rule_lhs,
    marpa_g_rule_length,
    marpa_g_rule_rhs,
    marpa_g_rule_is_proper_separation,
    marpa_g_sequence_min,
    marpa_g_sequence_new,
    marpa_g_sequence_separator,
    marpa_g_symbol_is_counted,
    marpa_g_rule_rank_set,
    marpa_g_rule_rank,
    marpa_g_rule_null_high_set,
    marpa_g_rule_null_high,
    MARPA_ERR_NONE,
    MARPA_PROPER_SEPARATION,
};

use result::{
    MarpaResult,
    err,
    err_code,
    err_nosym,
    err_norule,
    err_notaseq,
};

use std::mem::forget;
use std::ptr;

use std::ops::{
    Range,
    Deref,
};

pub struct Config {
    internal: Marpa_Config,
}

impl Config {
    pub fn new() -> Config {
        let mut cfg = Config { internal: Marpa_Config::default() };

        assert!(cfg.init() == MARPA_ERR_NONE);

        cfg
    }

    fn init(&mut self) -> Marpa_Error_Code {
        unsafe {
            marpa_c_init(&mut self.internal)
        }
    }

    pub fn error(&mut self) -> MarpaResult<()> {
        unsafe {
            match marpa_c_error(&mut self.internal, ptr::null_mut()) {
                0 => Ok(()),
                err if err > 0 => err_code(err),
                _ => err("error creating grammar"),
            }
        }
    }
}

pub struct Grammar {
    internal: Marpa_Grammar,
}

impl Clone for Grammar {
    fn clone(&self) -> Grammar {
        unsafe { marpa_g_ref(self.internal) };
        Grammar { internal: self.internal }
    }
}

impl Drop for Grammar {
    fn drop(&mut self) {
        forget(self.internal);
        unsafe {
            marpa_g_unref(self.internal);
        }
    }
}

impl Grammar {
    pub fn new(cfg: Config) -> MarpaResult<Grammar> {
        let mut cfg = cfg;
        unsafe {
            let c_grammar = marpa_g_new(&mut cfg.internal);

            try!(cfg.error());

            assert!(marpa_g_force_valued(c_grammar) >= 0);
            Ok(Grammar { internal: c_grammar })
        }
    }

    fn error(&self) -> MarpaResult<()> {
        match unsafe { marpa_g_error(self.internal, ptr::null_mut()) } {
            0 => Ok(()),
            code => err_code(code),
        }
    }

    fn error_or<S: Into<String>, T>(&self, s: S) -> MarpaResult<T> {
        match self.error() {
            Ok(()) => err(s),
            Err(error) => err(error),
        }
    }

    pub fn new_symbol(&mut self) -> MarpaResult<Symbol> {
        unsafe {
            match marpa_g_symbol_new(self.internal) {
                -2 => err("error creating new symbol"),
                sym => Ok(Symbol(sym)),
            }
        }
    }

    pub fn get_start_symbol(&self) -> MarpaResult<Symbol> {
        unsafe {
            match marpa_g_start_symbol(self.internal) {
                -1 => err_nosym(),
                -2 => err("error getting start symbol"),
                sym_id => Ok(Symbol(sym_id)),
            }
        }
    }

    pub fn set_start_symbol(&mut self, sym: Symbol) -> MarpaResult<Symbol> {
        unsafe {
            match marpa_g_start_symbol_set(self.internal, *sym) {
                -1 => err_nosym(),
                -2 => err("error setting start symbol"),
                sym_id => Ok(Symbol(sym_id)),
            }
        }
    }

    pub fn symbols(&self) -> MarpaResult<SymIter> {
        let max = unsafe { marpa_g_highest_symbol_id(self.internal) };
        match max {
            -2 => err("error getting highest symbol"),
            max => Ok(SymIter((0..max+1))),
        }
    }

    pub fn symbol_is_accessible(&self, sym: Symbol) -> MarpaResult<bool> {
        match unsafe { marpa_g_symbol_is_accessible(self.internal, *sym) } {
            1  => Ok(true),
            0  => Ok(false),
            -1 => err_nosym(),
            -2 => err("error checking symbol accessibility"),
            _  => panic!("unexpected error code"),
        }
    }

    pub fn symbol_is_nullable(&self, sym: Symbol) -> MarpaResult<bool> {
        match unsafe { marpa_g_symbol_is_nullable(self.internal, *sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => self.error_or("error checking symbol nullability"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_is_nulling(&self, sym: Symbol) -> MarpaResult<bool> {
        match unsafe { marpa_g_symbol_is_nulling(self.internal, *sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => err("error checking symbol nullness"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_is_productive(&self, sym: Symbol) -> MarpaResult<bool> {
        match unsafe { marpa_g_symbol_is_productive(self.internal, *sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => err("error checking symbol productivity"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_is_terminal(&self, sym: Symbol) -> MarpaResult<bool> {
        match unsafe { marpa_g_symbol_is_terminal(self.internal, *sym) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => err("error checking symbol terminality"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn symbol_set_terminal(&mut self, sym: Symbol, term: bool) -> MarpaResult<bool> {
        match unsafe { marpa_g_symbol_is_terminal_set(self.internal, *sym, term as i32) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err_nosym(),
            -2 => self.error_or("error setting symbol terminality"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn new_rule(&mut self, lhs: Symbol, rhs: Vec<Symbol>) -> MarpaResult<Rule> {
        let rhs_ptr = rhs.iter().map(|sym| sym.0).collect::<Vec<Marpa_Symbol_ID>>().as_mut_ptr();
        let rhs_len = rhs.len() as i32;
        match unsafe { marpa_g_rule_new(self.internal, *lhs, rhs_ptr, rhs_len) } {
            -2 => self.error_or("error creating new rule"),
            rule => Ok(Rule::BNF(rule)),
        }
    }

    pub fn rules(&self) -> MarpaResult<RuleIter> {
        let max = unsafe { marpa_g_highest_rule_id(self.internal) };
        match max {
            -2 => err("error getting highest symbol"),
            max => Ok(RuleIter((0..max+1), self.clone()))
        }
    }

    pub fn rule_is_accessible(&self, rule: Rule) -> MarpaResult<bool> {
        match unsafe { marpa_g_rule_is_accessible(self.internal, *rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule accessbility"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_is_nullable(&self, rule: Rule) -> MarpaResult<bool> {
        match unsafe { marpa_g_rule_is_nullable(self.internal, *rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule nullability"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_is_nulling(&self, rule: Rule) -> MarpaResult<bool> {
        match unsafe { marpa_g_rule_is_nulling(self.internal, *rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule nullness"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_is_loop(&self, rule: Rule) -> MarpaResult<bool> {
        match unsafe { marpa_g_rule_is_loop(self.internal, *rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule loop"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_is_productive(&self, rule: Rule) -> MarpaResult<bool> {
        match unsafe { marpa_g_rule_is_productive(self.internal, *rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule productivity"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_lhs(&self, rule: Rule) -> MarpaResult<Symbol> {
        match unsafe { marpa_g_rule_lhs(self.internal, *rule) } {
            -1 => err_norule(),
            -2 => self.error_or("error getting rule lhs"),
            symid => Ok(Symbol(symid)),
        }
    }

    pub fn rule_length(&self, rule: Rule) -> MarpaResult<i32> {
        match unsafe { marpa_g_rule_length(self.internal, *rule) } {
            -2 => self.error_or("error getting rule length"),
            len => Ok(len),
        }
    }

    pub fn rule_rhs_ix(&self, rule: Rule, ix: i32) -> MarpaResult<Symbol> {
        match unsafe { marpa_g_rule_rhs(self.internal, *rule, ix) } {
            -1 => err_norule(),
            -2 => self.error_or("error getting rhs ix"),
            symid => Ok(Symbol(symid)),
        }
    }

    pub fn rule_rhs(&self, rule: Rule) -> MarpaResult<Vec<Symbol>> {
        let len = try!(self.rule_length(rule));
        let mut syms: Vec<Symbol> = vec![];
        for id in (0..len) {
            let sym = try!(self.rule_rhs_ix(rule, id));

            syms.push(sym);
        }
        Ok(syms)
    }

    pub fn rule_is_proper_separation(&self, rule: Rule) -> MarpaResult<bool> {
        match unsafe { marpa_g_rule_is_proper_separation(self.internal, *rule) } {
            -1 => err_norule(),
            0 => Ok(false),
            1 => Ok(true),
            -2 => self.error_or("error getting rule separation"),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn sequence_min(&self, rule: Rule) -> MarpaResult<i32> {
        match unsafe { marpa_g_sequence_min(self.internal, *rule) } {
            -1 => err_notaseq(),
            -2 => self.error_or("error getting sequence min"),
            i => Ok(i),
        }
    }

    fn rule_is_sequence(&self, ruleid: i32) -> MarpaResult<bool> {
        match unsafe { marpa_g_sequence_min(self.internal, ruleid) } {
            -1 => Ok(false),
            -2 => self.error_or("error getting if sequence"),
            _ => Ok(true),
        }
    }

    pub fn new_sequence(&self,
                        lhs: Symbol,
                        rhs: Symbol,
                        sep: Symbol,
                        nonempty: bool,
                        proper: bool) -> MarpaResult<Rule> {
        match unsafe {
            marpa_g_sequence_new(self.internal,
                                 *lhs, *rhs, *sep,
                                 nonempty as i32,
                                 if proper {MARPA_PROPER_SEPARATION} else {0})
        } {
            -2 => self.error_or("error creating sequence"),
            ruleid => Ok(Rule::Seq(ruleid)),
        }
    }

    pub fn sequence_separator(&self, rule: Rule) -> MarpaResult<Rule> {
        match unsafe { marpa_g_sequence_separator(self.internal, *rule) } {
            -1 => err("Rule has no separator"),
            -2 => self.error_or("error getting sequence separator"),
            ruleid => if try!(self.rule_is_sequence(ruleid)) {
                Ok(Rule::Seq(ruleid))
            } else {
                Ok(Rule::BNF(ruleid))
            }
        }
    }

    pub fn symbol_is_counted(&self, sym: Symbol) -> MarpaResult<bool> {
        match unsafe { marpa_g_symbol_is_counted(self.internal, *sym) } {
            -1 => err_nosym(),
            -2 => self.error_or("error getting if symbol is counted"),
            0 => Ok(false),
            1 => Ok(true),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_rank_set(&mut self, rule: Rule, rank: i32) -> MarpaResult<()> {
        unsafe { marpa_g_rule_rank_set(self.internal, *rule, rank) };
        self.error()
    }

    pub fn rule_rank_get(&self, rule: Rule) -> MarpaResult<i32> {
        let rank = unsafe { marpa_g_rule_rank(self.internal, *rule) };
        if rank == -2 {
            match self.error() {
                Ok(()) => Ok(-2),
                Err(err) => Err(err),
            }
        } else {
            Ok(rank)
        }
    }

    pub fn rule_null_high_set(&mut self, rule: Rule, high: bool) -> MarpaResult<()> {
        match unsafe { marpa_g_rule_null_high_set(self.internal, *rule, high as i32) } {
            -1 => err_norule(),
            -2 => self.error_or("error setting null high"),
            c if c == 0 || c == 1 => Ok(()),
            _ => panic!("unexpected error code"),
        }
    }

    pub fn rule_null_high(&mut self, rule: Rule) -> MarpaResult<bool> {
        match unsafe { marpa_g_rule_null_high(self.internal, *rule) } {
            -1 => err_norule(),
            -2 => self.error_or("error setting null high"),
            0 => Ok(false),
            1 => Ok(true),
            _ => panic!("unexpected error code"),
        }
    }
}

pub struct SymIter(Range<i32>);

pub struct RuleIter(Range<i32>, Grammar);

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct Symbol(Marpa_Symbol_ID);

impl Deref for Symbol {
    type Target = Marpa_Symbol_ID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Marpa_Symbol_ID> for Symbol {
    fn from(other: Marpa_Symbol_ID) -> Symbol {
        Symbol(other)
    }
}

#[derive(Copy,Clone)]
pub enum Rule {
    BNF(Marpa_Rule_ID),
    Seq(Marpa_Rule_ID),
}

impl Deref for Rule {
    type Target = Marpa_Rule_ID;

    fn deref(&self) -> &Self::Target {
        match self {
            &Rule::BNF(ref i) => i,
            &Rule::Seq(ref i) => i,
        }
    }
}

#[cfg(test)]
mod tests {
    use thin::{
        Config,
        Grammar,
        Symbol,
        Rule,
    };

    fn new_grammar() -> Grammar {
        Grammar::new(Config::new()).unwrap()
    }

    #[test]
    fn create_grammar() {
        new_grammar();
    }

    #[test]
    fn iter_syms() {
        let mut g = new_grammar();
        for _ in (0..5) {
            g.new_symbol().unwrap();
        }
        assert!(g.symbols().unwrap().collect::<Vec<i32>>() == vec![0,1,2,3,4]);
    }

    #[test]
    fn iter_rules() {
        let mut g = new_grammar();
        for _ in (0..6) {
            g.new_symbol().unwrap();
        }

        g.new_rule(0.into(), vec![1.into(), 2.into()]).unwrap();
        g.new_rule(1.into(), vec![3.into(), 4.into()]).unwrap();
        g.new_rule(2.into(), vec![]).unwrap();

        assert!(g.rules().unwrap().collect::<Vec<i32>>() == vec![0,1, 2]);
        assert!(g.rule_is_accessible(Rule::BNF(0)).unwrap());
    }

    #[test]
    fn set_terminal() {
        let mut g = Grammar::new(Config::new()).unwrap();
        let s = g.new_symbol().unwrap();
        assert!(g.symbol_is_terminal(s).unwrap() == false);
        g.symbol_set_terminal(s, true).unwrap();
        assert!(g.symbol_is_terminal(s).unwrap() == true);
        let term = g.symbol_set_terminal(s, false);
        match term {
            Ok(_) => assert!(false),
            _ => {},
        }
    }

    #[test]
    fn set_start() {
        let mut g: Grammar = Grammar::new(Config::new()).unwrap();

        let sym: Symbol = g.new_symbol().unwrap();
        assert!(g.set_start_symbol(sym).unwrap() == sym);

        assert!(g.get_start_symbol().unwrap() == sym);
    }
}
