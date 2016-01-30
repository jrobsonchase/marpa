extern crate marpa;

use marpa::parser::*;
use marpa::parser::symbol::Symbol;
use marpa::lexer::byte_scanner::*;
use marpa::tree_builder::*;
use marpa::stack::*;

use std::io::Cursor;

// fn main() {
//     let mut p = Parser::new();
//     for _ in 0..256 { // create terminals
//         p.create_symbol().unwrap();
//     }

//     let start = p.create_symbol().unwrap();
//     let rule = p.create_symbol().unwrap();
//     let token_rule = p.create_symbol().unwrap();
//     let grammar_rule = p.create_symbol().unwrap();
//     let name = p.create_symbol().unwrap();
//     let ch = p.create_symbol().unwrap();
//     let space = p.create_symbol().unwrap();
//     let sep = p.create_symbol().unwrap();
//     let body = p.create_symbol().unwrap();
//     let token = p.create_symbol().unwrap();

//     let LPAREN: Symbol = '('.into();
//     let RPAREN: Symbol = ')'.into();
//     let TOKEN = Symbol::from_str("token");
//     let SEP = Symbol::from_str("::=");
//     let TERM: Symbol = ';'.into();
//     let SPACE: Symbol = ' '.into();
//     let LF: Symbol = '\n'.into();

//     let mut builder  = TreeBuilder::new();

//     p.add_rule(start, &[rule]).unwrap();
//     p.add_rule(start, &[rule, start]).unwrap();
//     p.add_rule(rule, &[token_rule]).unwrap();
//     p.add_rule(rule, &[grammar_rule]);
//     p.add_rule(grammar_rule, &[name, space, sep, space, body, space, TERM, LF]).unwrap();
//     p.add_rule(token_rule, &[token, space, grammar_rule]);
//     for i in 0..256 {
//         if i != ' ' as i32 {
//             p.add_rule(ch, &[i.into()]).unwrap();
//         }
//     }
//     p.add_rule(space, &[]).unwrap();
//     p.add_rule(name, &[ch]).unwrap();
//     p.add_rule(body, &[]).unwrap();
//     p.set_start(start).unwrap();

//     let mut t = p.run_recognizer(ByteScanner::new(Cursor::new("abc    ::= ;\ntoken herp ::= ;\n"))).unwrap();
//     let v = t.next().unwrap();


//     println!("{}", proc_value(builder, v));
// }


fn main() {
    let mut p = Parser::new();
    let mut b = TreeBuilder::new();
    for _ in 0..256 { // create terminals
        p.create_symbol().unwrap();
    }

    let LPAREN: Symbol = '('.into();
    let RPAREN: Symbol = ')'.into();
    let TEST = Symbol::from_str("test".into());

    let start = p.create_symbol().unwrap();
    let expr = p.create_symbol().unwrap();
    let lparen = p.create_symbol().unwrap();
    let rparen = p.create_symbol().unwrap();
    let test = p.create_symbol().unwrap();


    let start_rule = p.add_rule(start, &[expr]).unwrap();
    let lparen_rule = p.add_rule(lparen, &[LPAREN]).unwrap();
    let rparen_rule = p.add_rule(rparen, &[RPAREN]).unwrap();
    let test_rule = p.add_rule(test, &TEST).unwrap();
    let expr_1 = p.add_rule(expr, &[lparen, expr, rparen]).unwrap();
    let expr_2 = p.add_rule(expr, &[test]).unwrap();

    for r in &[start_rule, expr_1, expr_2] {
        b.rule(**r);
    }

    for t in &[lparen_rule, rparen_rule, test_rule] {
        b.token(**t);
    }

    p.set_start(start).unwrap();

    let mut t = p.run_recognizer(ByteScanner::new(Cursor::new("((((test))))"))).unwrap();
    let v = t.next().unwrap();


    println!("{}", proc_value(TreeBuilder::new(), v));
}
