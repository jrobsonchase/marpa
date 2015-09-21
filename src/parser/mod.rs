use lexer::Lexer;

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

pub struct Parser {
    lex: Lexer,
    parse: MarpaState,
}

impl Parser {
    pub fn new(lex: Lexer, grammar: Grammar) -> Self {
        Parser{ lex: lex, parse: G(grammar) }
    }

    fn add_rule(&mut self, lhs: Symbol, rhs: &[Symbol]) -> Result<Rule> {
        match self.parse {
            G(ref mut g) => {
                g.new_rule(lhs, rhs)
            },
            _ => Err("Marpa is not in the Grammar state".into()),
        }
    }

    fn adv_marpa(&mut self) -> Result<()> {
        self.parse = try!(self.parse.adv());
        Ok(())
    }

    fn run_recognizer(&mut self, input: &str) -> Result<()> {
        match self.parse {
            R(ref mut r) => {
                try!(r.start_input());
                loop {
                    if r.is_exhausted() {
                        break;
                    }
                    let maybe_tok = self.lex.next_token(input);
                    match maybe_tok {
                        None => panic!("no viable input"),
                        Some((tok, val)) => {
                            try!(r.alternative(tok.ty as i32, val as i32, 1));
                            try!(r.earleme_complete());
                        }
                    }
                }
            },
            _ => return Err("Marpa is not in the Recognizer state".into()),
        }
        loop {
            try!(self.adv_marpa());
            if let &T(_) = &self.parse {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use lexer::LexGen;
    use parser::Parser;

    use thin::*;

    #[test]
    fn stuff() {
        let input = "this is a test";
        let lex = {
            let mut gen = LexGen::new();
            for i in vec![r"is", r"test", r"this", r"a|b", r" "] {
                gen.new_re(i).unwrap();
            }
            gen.compile()
        };

        let mut g = Grammar::new().unwrap();
        let is = g.new_symbol().unwrap();
        let test = g.new_symbol().unwrap();
        let this = g.new_symbol().unwrap();
        let a = g.new_symbol().unwrap();
        let space = g.new_symbol().unwrap();
        let start = g.new_symbol().unwrap();

        g.set_start_symbol(start).unwrap();

        let start_rule = g.new_rule(start, &[this, space, is, space, a, space, test]).unwrap();

        g.precompute().unwrap();

        let mut parse = Parser::new(lex, g);

        parse.adv_marpa().unwrap();

        parse.run_recognizer(input).unwrap();
    }
}
