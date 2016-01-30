use std::mem;

use lexer::token_source::TokenSource;
use lexer::token::Token;

use result::Result;

pub mod symbol;
use self::symbol::Symbol;

mod rule;
use self::rule::Rule;

use thin::{
    Grammar,
    Recognizer,
    Bocage,
    Order,
    Tree,
    // Value,
};

#[allow(dead_code)]
enum MarpaState {
    G(Grammar),
    R(Recognizer),
    B(Bocage),
    O(Order),
    T(Tree),
}

use self::MarpaState::{
    G,
    R,
    B,
    O,
    T,
};

impl MarpaState {
    fn new() -> Self {
        G(Grammar::new().unwrap())
    }

    fn adv(&mut self) -> Result<MarpaState> {
        match self {
            &mut G(ref mut g) => {
                try!(g.precompute());
                Recognizer::new(g.clone()).map(|s| R(s))
            },
            &mut R(ref r) => Bocage::new(r.clone()).map(|s| B(s)),
            &mut B(ref b) => Order::new(b.clone()).map(|s| O(s)),
            &mut O(ref o) => Tree::new(o.clone()).map(|s| T(s)),
            &mut T(_) => Err("No next state".into()),
        }
    }
}

impl Default for MarpaState {
    fn default() -> Self {
        MarpaState::new()
    }
}

pub struct Parser {
    state: MarpaState,
}

macro_rules! get_state {
    ($e:expr, $s:ident) => ({
        match $e.state {
            $s(ref mut g) => g,
            _ => return Err(format!("Marpa is not in the {} state", stringify!($s)).into()),
        }
    })
}

impl Parser {
    pub fn new() -> Self {
        Parser{ state: Default::default() }
    }

    pub fn create_symbol(&mut self) -> Result<Symbol> {
        get_state!(self, G).new_symbol().map(|x| x.into())
    }

    pub fn set_start(&mut self, sym: Symbol) -> Result<Symbol> {
        get_state!(self, G).set_start_symbol(*sym).map(|x| x.into())
    }

    pub fn add_rule(&mut self, lhs: Symbol, rhs: &[Symbol]) -> Result<Rule> {
        // using transmute here to avoid an extra copy. if Symbol is ever changed to
        // not just be a Symbol(thin::Symbol), this will have some crazy results.
        get_state!(self, G).new_rule(*lhs, unsafe { mem::transmute(rhs) }).map(|x| x.into())
    }

    pub fn add_seq(&mut self, lhs: Symbol, rhs: Symbol, sep: Symbol, nonempty: bool, proper: bool) -> Result<Rule> {
        get_state!(self, G).new_sequence(*lhs, *rhs, *sep, nonempty, proper).map(|x| x.into())
    }

    fn adv_marpa(&mut self) -> Result<()> {
        self.state = try!(self.state.adv());
        Ok(())
    }

    pub fn run_recognizer<T: TokenSource<U>, U: Token>(&mut self, tokens: T) -> Result<Tree> {
        let mut tokens = tokens;
        if let G(_) = self.state {
            try!(self.adv_marpa())
        }
        { // limit recognizer borrow
            let r = get_state!(self, R);
            try!(r.start_input());
            loop {
                if r.is_exhausted() {
                    break;
                }
                let maybe_tok = tokens.next();
                match maybe_tok {
                    None => break,
                    Some(tok) => {
                        try!(r.alternative(tok.sym(), tok.value(), 1));
                        try!(r.earleme_complete());
                    }
                }
            }
        }
        loop {
            try!(self.adv_marpa());
            if let T(ref tree) = self.state {
                return Ok(tree.clone());
            }
        }
    }
}

