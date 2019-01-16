use crate::lexer::token::Token;
use crate::lexer::token_source::TokenSource;

use crate::result::Result;

use crate::thin::{
    Bocage,
    Grammar,
    Order,
    Recognizer,
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

use self::MarpaState::{B, G, O, R, T};

impl MarpaState {
    fn new() -> Self {
        G(Grammar::new().unwrap())
    }

    fn adv(&mut self) -> Result<MarpaState> {
        match self {
            G(ref mut g) => {
                g.precompute()?;
                Recognizer::new(g.clone()).map(R)
            }
            R(ref r) => Bocage::new(r.clone()).map(B),
            B(ref b) => Order::new(b.clone()).map(O),
            O(ref o) => Tree::new(o.clone()).map(T),
            T(_) => Err("No next state".into()),
        }
    }
}

impl Default for MarpaState {
    fn default() -> Self {
        MarpaState::new()
    }
}

#[derive(Default)]
pub struct Parser {
    state: MarpaState,
}

macro_rules! get_state {
    ($e:expr, $s:ident) => {{
        match $e.state {
            $s(ref mut g) => g,
            _ => return Err(format!("Marpa is not in the {} state", stringify!($s)).into()),
        }
    }};
}

impl Parser {
    pub fn new() -> Self {
        Parser::default()
    }

    pub fn with_grammar(g: Grammar) -> Self {
        Parser { state: G(g) }
    }

    fn adv_marpa(&mut self) -> Result<()> {
        self.state = self.state.adv()?;
        Ok(())
    }

    pub fn run_recognizer<T: TokenSource<U>, U: Token>(&mut self, tokens: T) -> Result<Tree> {
        let mut tokens = tokens;
        if let G(_) = self.state {
            self.adv_marpa()?
        }
        {
            // limit recognizer borrow
            let r = get_state!(self, R);
            r.start_input()?;
            loop {
                if r.is_exhausted() {
                    break;
                }
                let maybe_tok = tokens.next();
                match maybe_tok {
                    None => break,
                    Some(tok) => {
                        Parser::consume_tok(r, tok)?;
                    }
                }
            }
        }
        loop {
            self.adv_marpa()?;
            if let T(ref tree) = self.state {
                return Ok(tree.clone());
            }
        }
    }

    fn consume_tok<U: Token>(r: &mut Recognizer, tok: U) -> Result<()> {
        r.alternative(tok.sym(), tok.value(), 1)?;
        r.earleme_complete()?;
        Ok(())
    }
}
