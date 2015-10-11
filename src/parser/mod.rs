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
            &mut G(ref mut g) => Recognizer::new(g.clone()).map(|s| R(s)),
            &mut R(ref r) => Bocage::new(r.clone()).map(|s| B(s)),
            &mut B(ref b) => Order::new(b.clone()).map(|s| O(s)),
            &mut O(ref o) => Tree::new(o.clone()).map(|s| T(s)),
            &mut T(ref t) => Err("No next state".into()),
        }
    }
}

pub struct Parser<T: TokenSource> {
    state: MarpaState,
    tokens: T
}

impl<T: TokenSource> Parser<T> {
    pub fn new(tokens: T, grammar: Grammar) -> Self {
        Parser{ tokens: tokens, state: G(grammar) }
    }

    fn add_rule(&mut self, lhs: Symbol, rhs: &[Symbol]) -> Result<Rule> {
        match self.state {
            G(ref mut g) => {
                g.new_rule(lhs, rhs)
            },
            _ => Err("Marpa is not in the Grammar state".into()),
        }
    }

    fn adv_marpa(&mut self) -> Result<()> {
        self.state = try!(self.state.adv());
        Ok(())
    }

    fn run_recognizer(&mut self) -> Result<()> {
        match self.state {
            R(ref mut r) => {
                try!(r.start_input());
                loop {
                    if r.is_exhausted() {
                        break;
                    }
                    let maybe_tok = self.tokens.next_token();
                    match maybe_tok {
                        None => break,
                        Some(tok) => {
                            try!(r.alternative(tok.ty as i32, tok.val as i32, 1));
                            try!(r.earleme_complete());
                        }
                    }
                }
            },
            _ => return Err("Marpa is not in the Recognizer state".into()),
        }
        loop {
            try!(self.adv_marpa());
            if let &T(_) = &self.state {
                break;
            }
        }
        Ok(())
    }
}

