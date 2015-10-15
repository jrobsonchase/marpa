extern crate marpa;

use marpa::thin::*;
use marpa::parser::*;
use marpa::lexer::token_source::*;
use marpa::lexer::token::*;
use marpa::lexer::byte_scanner::*;
use marpa::tree_builder::*;
use marpa::stack::*;

use std::io::Cursor;

fn main() {
    let mut g = Grammar::new().unwrap();
    for _ in 0..256 { // create terminals
        g.new_symbol().unwrap();
    }

    let start = g.new_symbol().unwrap();
    let item = g.new_symbol().unwrap();
    let butts = g.new_symbol().unwrap();
    g.new_rule(start, &[item]).unwrap();
    g.new_rule(item, &[item, '(' as Symbol, item, ')' as Symbol, item]).unwrap();
    g.new_rule(item, &[]).unwrap();
    g.new_rule(item, &[butts]);
    g.new_rule(butts, &as_syms("butts")).unwrap();
    g.set_start_symbol(start).unwrap();
    g.precompute().unwrap();
    let mut r = Recognizer::new(g).unwrap();
    r.start_input().unwrap();

    for tok in ByteScanner::new(Cursor::new("(butts)(()butts((butts(())))()butts)")) {
        r.alternative(tok.ty, tok.val, 1).unwrap();
        r.earleme_complete().unwrap();
    }

    let b = Bocage::new(r).unwrap();
    let o = Order::new(b).unwrap();
    let mut t = Tree::new(o).unwrap();
    let v = t.next().unwrap();

    println!("{}", proc_value(TreeBuilder::new(), v));
}

fn as_syms(s: &str) -> Vec<Symbol> {
    s.as_bytes().iter().map(|b| *b as Symbol).collect()
}
