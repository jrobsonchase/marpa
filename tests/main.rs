#![allow(non_snake_case,unused_variables)]

extern crate marpa;

use marpa::parser::*;
use marpa::lexer::byte_scanner::*;
use marpa::tree_builder::*;
use marpa::stack::*;
use marpa::grammar::Grammar;
use marpa::result::Result;

use std::io::Cursor;

#[test]
fn main() {
    real_main().unwrap();
}

fn real_main() -> Result<()> {
    let mut g = try!(Grammar::new());

    let ws_char = try!(g.string_set(None, "\t\n\r "));
    let ws = try!(g.star(None, ws_char));
    let sep = try!(g.literal_string(None, "::="));
    let term = try!(g.literal_string(None, ";"));

    let dquote = try!(g.literal_string(None, "\""));
    let squote = try!(g.literal_string(None, "'"));
    let escape = try!(g.literal_string(None, "\\"));
    let not_dquote = try!(g.inverse_string_set(None, "\""));
    let not_squote = try!(g.inverse_string_set(None, "'"));
    let dquote_escape = try!(g.rule(None, &[escape, dquote]));
    let squote_escape = try!(g.rule(None, &[escape, squote]));
    let str_chars = try!(g.alternative(None, &[not_dquote, dquote_escape]));
    let str_chars_star = try!(g.star(None, str_chars));
    let string = try!(g.rule(None, &[dquote, str_chars_star, dquote]));
    let char_chars = try!(g.alternative(None, &[not_squote, squote_escape]));

    let lower = try!(g.char_range(None, 'a', 'z'));
    let upper = try!(g.char_range(None, 'A', 'Z'));
    let digit = try!(g.char_range(None, '0', '9'));

    let alpha_num = try!(g.alternative(None, &[lower, upper, digit]));

    let ident = try!(g.plus(None, alpha_num));

    let rule = try!(g.rule(None, &[ident, ws, sep, ws, string, ws, term]));
    let rules = try!(g.sequence(None, rule, ws_char, false, false));

    let start = rules;

    try!(g.set_start(start));

    let mut b = TreeBuilder::new();

    for r in [start, rules].iter().map(|x| x.rule()) {
        b.rule(r);
    }

    for t in [ident, string, sep, term].iter().map(|x| x.rule()) {
        b.token(t);
    }

    for d in [ws_char].iter().map(|x| x.rule()) {
        b.discard(d);
    }

    let mut p = Parser::with_grammar(g.unwrap());

    let mut t = try!(p.run_recognizer(ByteScanner::new(Cursor::new("a ::= \"test\";"))));
    let v = t.next().unwrap();

    println!("{}", proc_value(b, v));
    Ok(())
}
