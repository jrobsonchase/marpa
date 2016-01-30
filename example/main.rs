#![allow(non_snake_case)]

extern crate marpa;

use marpa::parser::*;
use marpa::parser::symbol::Symbol;
use marpa::lexer::byte_scanner::*;
use marpa::tree_builder::*;
use marpa::stack::*;

use std::io::Cursor;

fn main() {
    let mut p = Parser::new();
    for _ in 0..256 { // create terminals
        p.create_symbol().unwrap();
    }

    let LPAREN: Symbol = '('.into();
    let RPAREN: Symbol = ')'.into();
    let TEST = Symbol::from_str("test".into());

    let start = p.create_symbol().unwrap();
    let expr = p.create_symbol().unwrap();
    let sub_expr_1 = p.create_symbol().unwrap();
    let sub_expr_2 = p.create_symbol().unwrap();
    let lparen = p.create_symbol().unwrap();
    let rparen = p.create_symbol().unwrap();
    let test = p.create_symbol().unwrap();


    let start_rule = p.add_rule(start, &[expr]).unwrap();
    let lparen_rule = p.add_rule(lparen, &[LPAREN]).unwrap();
    let rparen_rule = p.add_rule(rparen, &[RPAREN]).unwrap();
    let test_rule = p.add_rule(test, &TEST).unwrap();
    let expr_1_rule = p.add_rule(expr, &[sub_expr_1, expr, sub_expr_2]).unwrap();
    let expr_2_rule = p.add_rule(expr, &[test]).unwrap();
    let sub_expr_1_rule = p.add_rule(sub_expr_1, &[lparen]).unwrap();
    let sub_expr_2_rule = p.add_rule(sub_expr_2, &[rparen]).unwrap();

    p.set_start(start).unwrap();

    let mut b = TreeBuilder::new();

    for r in &[start_rule, expr_1_rule, expr_2_rule] {
        b.rule(**r);
    }

    for t in &[lparen_rule, rparen_rule, test_rule] {
        b.token(**t);
    }

    let mut t = p.run_recognizer(ByteScanner::new(Cursor::new("((((test))))"))).unwrap();
    let v = t.next().unwrap();

    println!("{}", proc_value(b, v));
}
