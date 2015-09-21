extern crate regex;
extern crate regex_syntax;

use self::regex::Regex;

use self::regex_syntax::Expr;

use std::slice::Iter;

pub struct Lexer {
    re: Regex,
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Default)]
pub struct LexGen {
    re_strings: Vec<String>,
}

impl LexGen {
    pub fn new() -> LexGen {
        LexGen::default()
    }

    pub fn new_re(&mut self, s: &str) -> Result<(), regex::Error> {
        self.re_strings.push(try!(remove_captures(s)));
        Ok(())
    }

    pub fn compile(&self) -> Lexer {
        let mut re_buf = String::new();
        for re in self.re_strings.iter() {
            re_buf.push('(');
            re_buf.push('^');
            re_buf.push_str(re);
            re_buf.push(')');
            re_buf.push('|');
        }

        re_buf.pop();

        println!("re_buf: {}", re_buf);

        Lexer { re: Regex::new(&re_buf).unwrap(), tokens: Vec::new(), pos: 0 }
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
/// A lexed token - includes a start, end, and type. The type refers to
// the regexp index that matched the token.
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub ty: usize,
}


impl Lexer {
    pub fn token(&self, ix: usize) -> Option<Token> {
        self.tokens.get(ix - 1).map(|t| *t)
    }

    pub fn tokens(&self) -> Iter<Token> {
        self.tokens.iter()
    }

    pub fn token_text<'a>(&self, tok: Token, input: &'a str) -> &'a str {
        &input[tok.start .. tok.end]
    }

    fn remaining_text<'a>(&self, input: &'a str) -> &'a str {
        &input[self.pos..]
    }

    fn new_token(&mut self, len: usize, ty: usize) -> (Token, usize) {
        let val = self.tokens.len();
        let start = self.pos;
        self.pos += len;
        let tok = Token{ start: start, end: self.pos, ty: ty };
        self.tokens.push(tok);
        (tok, val + 1)
    }

    /// Maybe return a token. If there is a successful match, return the
    /// the next token and its value index, otherwise, return None.
    pub fn next_token(&mut self, input: &str) -> Option<(Token, usize)> {
        { // limit length of self borrow for cap
            let cap = self.re.captures(self.remaining_text(input));

            let caps = if let Some(caps) = cap {
                caps
            } else {
                return None;
            };

            for i in (1..caps.len()) {
                if let Some((_, len)) = caps.pos(i) {
                    return Some(self.new_token(len, i-1))
                }
            }
        }

        None
    }
}

fn remove_captures_expr(expr: &mut Expr) {
    match expr {
        &mut Expr::Group{ ref mut i, ref mut e, ref mut name} => {
            i.take();
            name.take();
            remove_captures_expr(e);
        }
        &mut Expr::Repeat{ ref mut e, r: _, greedy: _ } => {
            remove_captures_expr(e);
        }

        &mut Expr::Concat(ref mut exprs) | &mut Expr::Alternate(ref mut exprs) => {
            for e in exprs.iter_mut() {
                remove_captures_expr(e);
            }
        }
        _ => {},
    }
}

fn remove_captures(expr: &str) -> Result<String, regex_syntax::Error> {
    let mut r_expr = try!(Expr::parse(expr));
    remove_captures_expr(&mut r_expr);
    Ok(format!("{}", r_expr))
}

#[cfg(test)]
mod tests {
    extern crate regex_syntax;

    use lexer::LexGen;

    #[test]
    fn asdf() {
        let input = "this is a test";
        let mut lex = {
            let mut gen = LexGen::new();
            for i in vec![r"is", r"test", r"this", r"a|b", r" "] {
                gen.new_re(i).unwrap();
            }
            gen.compile()
        };

        while let Some(_) = lex.next_token(input) {
        }

        let types: Vec<usize> = lex.tokens().map(|t| t.ty).collect();
        let texts: Vec<&str> = lex.tokens().map(|t| lex.token_text(*t, input)).collect();

        assert!(types == vec![
            2,
            4,
            0,
            4,
            3,
            4,
            1,
            ]);
        assert!(texts == vec![
            "this",
            " ",
            "is",
            " ",
            "a",
            " ",
            "test",
            ]);
    }
}
