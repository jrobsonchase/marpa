use libmarpa_sys::{
    Marpa_Grammar,
    Marpa_Config,
    Marpa_Error_Code,
    Marpa_Symbol_ID,
    Marpa_Rule_ID,
    marpa_c_init,
    marpa_c_error,
    marpa_g_new,
    marpa_g_force_valued,
    marpa_g_unref,
    marpa_g_symbol_new,
    marpa_g_start_symbol,
    marpa_g_start_symbol_set,
    marpa_g_highest_symbol_id,
    marpa_g_symbol_is_accessible,
    MARPA_ERR_NONE,
};

use result::{
    MarpaResult,
    ErrNotOk,
    ErrNoSym,
    err,
    err_code,
};

use std::mem::forget;
use std::ptr;

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
                29 => err(ErrNotOk),
                err => err_code(err),
            }
        }
    }
}

pub struct Grammar {
    internal: Marpa_Grammar,
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

    pub fn new_symbol(&mut self) -> MarpaResult<Symbol> {
        unsafe {
            match marpa_g_symbol_new(self.internal) {
                -2 => err("error creating new symbol"),
                sym => Ok(Symbol(sym)),
            }
        }
    }

    pub fn get_start(&self) -> MarpaResult<Symbol> {
        unsafe {
            match marpa_g_start_symbol(self.internal) {
                -1 => err(ErrNoSym),
                -2 => err("error getting start symbol"),
                sym_id => Ok(Symbol(sym_id)),
            }
        }
    }

    pub fn set_start(&mut self, sym: Symbol) -> MarpaResult<Symbol> {
        unsafe {
            match marpa_g_start_symbol_set(self.internal, sym.0) {
                -1 => err(ErrNoSym),
                -2 => err("error setting start symbol"),
                sym_id => Ok(Symbol(sym_id)),
            }
        }
    }

    pub fn symbols(&self) -> MarpaResult<SymIter> {
        let mut max = unsafe { marpa_g_highest_symbol_id(self.internal) };
        match max {
            -2 => err("error getting highest symbol"),
            max => Ok(SymIter{ grammar: self, current: 0, max: max })
        }
    }

    pub fn is_accessible(&self, sym: Symbol) -> MarpaResult<bool> {
        match unsafe { marpa_g_symbol_is_accessible(self.internal, sym.0) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => err(ErrNoSym),
            -2 => err("error checking symbol accessibility"),
            _ => panic!("unexpected error code"),
        }
    }
}

pub struct SymIter<'a> {
    grammar: &'a Grammar,
    max: i32,
    current: i32,
}

impl<'a> Iterator for SymIter<'a> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.max {
            let tmp = self.current;
            self.current += 1;
            Some(Symbol(tmp))
        } else {
            None
        }
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

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct Symbol(Marpa_Symbol_ID);

#[derive(Clone)]
pub enum Rule {
    BNF(Marpa_Rule_ID),
    Seq(Marpa_Rule_ID),
}