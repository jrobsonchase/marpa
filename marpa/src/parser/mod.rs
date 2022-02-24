use std::mem;

use crate::lexer::token::Token;
use crate::lexer::token_source::TokenSource;
use crate::asf::{ASF, Traverser};
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
    G,
    GReady,
    R(Recognizer),
    B(Bocage),
    O(Order),
    T(Tree),
}

use self::MarpaState::{GReady, B, G, O, R, T};

impl MarpaState {
    fn new() -> Self {
        G
    }
}

impl Default for MarpaState {
    fn default() -> Self {
        MarpaState::new()
    }
}

pub struct Parser {
    grammar: Grammar,
    state: MarpaState,
}
impl Default for Parser {
    fn default() -> Self {
        Parser {
            state: MarpaState::default(),
            grammar: Grammar::new().unwrap(),
        }
    }
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

    pub fn with_grammar(grammar: Grammar) -> Self {
        Parser { state: G, grammar }
    }

    fn adv_marpa(&mut self) -> Result<()> {
        let next_state = match self.state {
            G => {
                self.grammar.precompute()?;
                Recognizer::new(self.grammar.clone()).map(R)
            }
            GReady => Recognizer::new(self.grammar.clone()).map(R),
            R(ref r) => Bocage::new(&r).map(B),
            B(ref b) => {
                Order::new(&b).map(O)
            },
            O(ref o) => Tree::new(o.clone()).map(T),
            T(_) => Ok(GReady),
        };
        self.state = next_state?;
        Ok(())
    }

    pub fn read<T: TokenSource<U>, U: Token>(&mut self, mut tokens: T) -> Result<()> {
        loop {
            // just prep the recognizer, irrespective of initial state
            match self.state {
                R(_) => break,
                _ => self.adv_marpa()?,
            }
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
        Ok(())
    }
    pub fn run_recognizer<T: TokenSource<U>, U: Token>(&mut self, tokens: T) -> Result<Tree> {
        self.read(tokens)?;
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

    /// This is roughly equivalent to `$asf->traverse` in Marpa::R2,
    /// but the ASF details are hidden under the hood.
    pub fn parse_and_traverse_forest<T: TokenSource<U>, U: Token, PT, PS>(&mut self, tokens: T, init_state:PS, traverser: Box<dyn Traverser<ParseTree = PT, ParseState=PS>>) -> Result<(PT, PS)> {
        // we need to read the tokens before starting the ASF step
        self.read(tokens)?;
        if let R(recce) = mem::replace(&mut self.state, GReady) {
            let mut asf = ASF::new(recce)?;
            asf.traverse(init_state, traverser)
        } else {
            panic!("Parser::read must always terminate in the R state!");
        }
    }

}
