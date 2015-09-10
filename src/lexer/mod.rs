extern crate regex;

use self::regex::Regex;

use std::borrow::Borrow;

pub struct Lexer {
    input: String,
    tokens: Vec<Token>,
    tok_re: Vec<Regex>,
}

#[derive(Copy,Clone)]
pub struct Token {
    start: usize,
    end: usize,
    ty: i32,
}

impl Lexer {
    pub fn new<S: Into<String>>(s: S) -> Lexer {
        Lexer{ input: s.into(), tokens: Vec::new(), tok_re: Vec::new() }
    }

    pub fn new_re(&mut self, s: &str) -> Result<(), regex::Error> {
        self.tok_re.push(try!(Regex::new(s)));
        Ok(())
    }

    pub fn token(&self, ix: usize) -> Option<&Token> {
        self.tokens.get(ix)
    }

    pub fn token_text(&self, tok: Token) -> &str {
        let borrowed: &str = self.input.borrow();
        &borrowed[tok.start .. tok.end]
    }

    fn remaining_text(&self) -> &str {
        match self.tokens.last() {
            None => self.input.borrow(),
            Some(tok) => {
                let borrowed: &str = self.input.borrow();
                &borrowed[tok.end..]
            },
        }
    }

    fn new_token(&mut self, end: usize, ty: i32) -> (Token, i32) {
        let val = self.tokens.len();
        let start = if val > 0 { self.tokens.last().unwrap().end } else { 0 };
        self.tokens.push(Token{ start: start, end: end, ty: ty });
        (self.tokens.last().unwrap().clone(), val as i32)
    }

    pub fn next_token(&mut self) -> Option<(Token, i32)> {
        for i in (0..self.tok_re.len()) {
            match self.tok_re.get(i).unwrap().find(self.remaining_text()) {
                None => {},
                Some((_, end)) => return Some(self.new_token(end, i as i32)),
            }
        }
        None
    }

    pub fn next_token_hinted<I: IntoIterator<Item=usize>>(&mut self, it: I) -> Option<(Token, i32)> {
        for ix in it {
            if ix < self.tok_re.len() {
                match self.tok_re.get(ix).unwrap().find(self.remaining_text()) {
                    None => {},
                    Some((_, end)) => return Some(self.new_token(end, ix as i32)),
                }
            }
        }
        None
    }

    pub fn tokens<'a>(&'a mut self) -> TokIter<'a> {
        TokIter{lex: self}
    }
}

pub struct TokIter<'a> {
    lex: &'a mut Lexer,
}

impl<'a> Iterator for TokIter<'a> {
    type Item = (Token, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.lex.next_token()
    }
}
