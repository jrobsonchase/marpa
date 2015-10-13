use lexer::token_source::TokenSource;

use thin::*;

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

pub struct Parser<T: TokenSource> {
    state: MarpaState,
    tokens: T
}

macro_rules! get_state {
    ($e:expr, $s:ident) => ({
        match $e.state {
            $s(ref mut g) => g,
            _ => return Err(format!("Marpa is not in the {} state", stringify!($s)).into()),
        }
    })
}

impl<T: TokenSource> Parser<T> {
    pub fn new(tokens: T) -> Self {
        Parser{ tokens: tokens, state: Default::default() }
    }

    pub fn set_start(&mut self, sym: Symbol) -> Result<Symbol> {
        get_state!(self, G).set_start_symbol(sym)
    }
    pub fn add_rule(&mut self, lhs: Symbol, rhs: &[Symbol]) -> Result<Rule> {
        get_state!(self, G).new_rule(lhs, rhs)
    }

    pub fn add_seq(&mut self, lhs: Symbol, rhs: Symbol, sep: Symbol, nonempty: bool, proper: bool) -> Result<Rule> {
        get_state!(self, G).new_sequence(lhs, rhs, sep, nonempty, proper)
    }

    fn adv_marpa(&mut self) -> Result<()> {
        self.state = try!(self.state.adv());
        Ok(())
    }

    pub fn run_recognizer(&mut self) -> Result<Tree> {
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
                let maybe_tok = self.tokens.next_token();
                match maybe_tok {
                    None => break,
                    Some(tok) => {
                        try!(r.alternative(tok.ty, tok.val, 1));
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

