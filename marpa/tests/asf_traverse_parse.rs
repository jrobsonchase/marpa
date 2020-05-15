extern crate marpa;

use marpa::grammar::Grammar;
use marpa::lexer::byte_scanner::*;
use marpa::parser::*;
use marpa::result::Result;
use marpa::stack::*;
use marpa::tree_builder::*;
use marpa::asf::{Glade, Traverser};

use std::io::Cursor;
use std::rc::Rc;
use std::collections::HashMap;

const PANDA_INPUT : &str = "a panda eats shoots and leaves.";

#[test]
fn recce_parse_sanity() {
  // check that recce behaves as expected, for sanity's sake
  let (mut parser, b, rule_names) = build_grammar().expect("grammar build should succeed, not core part of test");

  let mut parsed_result_iterator = parser.run_recognizer(ByteScanner::new(Cursor::new(PANDA_INPUT))).expect("recognizer suceeds");
  let mut parse_count = 0;
  while let Some(v) = parsed_result_iterator.next() {
    // println!("{}", proc_value(b.clone(), v));
    parse_count += 1;
  }
  assert_eq!(parse_count, 3, "panda sentence should have three parses with run_recognizer.");
}

#[test]
fn asf_traverse_parse() {
  let runner_result = runner_asf_traverse();
  assert!(runner_result.is_ok(), format!("failed to run asf traversal: {:?}", runner_result.err()));
}

fn runner_asf_traverse() -> Result<Vec<String>> {
  let (mut parser, b, rule_names) = build_grammar().expect("grammar build should succeed, not core part of test");
  // Now that we have validated the panda grammar is correctly ambiguous,
  // reparse it via the ASFs
  let mut parse_forest_iterator = parser.parse_and_traverse_forest(
    ByteScanner::new(Cursor::new(PANDA_INPUT)),
    (),//init state
    Box::new(ExhaustiveTraverser { rule_names: rule_names.clone() })
  )?;

  let mut parse_forest_iterator = parser.parse_and_traverse_forest(
    ByteScanner::new(Cursor::new(PANDA_INPUT)),
    (),//init state
    Box::new(PruningTraverser { rule_names })
  )?;

  Ok(Vec::new())
}

// Do a standalone build for each test, to avoid reentrance errors
fn build_grammar() -> Result<(Parser, TreeBuilder, HashMap<i32, &'static str>)> {
  let mut g = Grammar::new()?;
  let mut b = TreeBuilder::new();

  let ws = g.string_set(None, "\t\n\r ")?;
  //b.discard(ws.rule());

  let period = g.literal_string(None, ".")?;
  let cc = g.literal_string(None, "and")?;
  let det1 = g.literal_string(None, "a")?;
  let det2 = g.literal_string(None, "an")?;
  let dt = g.alternative(None, &[det1, det2])?;
  let panda = g.literal_string(None, "panda")?;
  let eats = g.literal_string(None, "eats")?;
  let shoots = g.literal_string(None, "shoots")?;
  let leaves = g.literal_string(None, "leaves")?;

  let nns = g.alternative(None, &[shoots, leaves])?;
  let vbz = g.alternative(None, &[eats, shoots, leaves])?;

  let nn = g.rule(None, &[panda])?;
  let np = g.rule(None, &[nn])?;
  let _np_simple_2 = g.rule(Some(np), &[nns])?;
  let _np_compound_1 = g.rule(Some(np), &[dt, ws, nn])?;
  let _np_compound_2 = g.rule(Some(np), &[nn, ws, nns])?;
  let _np_compound_3 = g.rule(Some(np), &[nns, ws, cc, ws, nns])?;

  let vp = g.rule(None, &[vbz])?;
  let _vp_1 = g.rule(Some(vp), &[vbz, ws, np])?;
  let _vp_2 = g.rule(Some(vp), &[vp, ws, vbz, ws, nns])?;
  let _vp_3 = g.rule(Some(vp), &[vp, ws, cc, ws, vp])?;
  let _vp_4 = g.rule(Some(vp), &[vp, ws, vp, ws, cc, ws, vp])?;

  let s = g.rule(None, &[np, ws, vp, period])?;
  g.set_start(s)?;

  // for t_rule in &[cc, det1, det2, panda, eats, shoots, leaves] {
  //   b.token(t_rule.rule());
  // }
  // for r in &[nn, dt, nns, vbz, np, vp, s] {
  //   b.rule(r.rule());
  // }

  let mut rule_names = HashMap::new();
  rule_names.insert(np.rule(),"NP");
  rule_names.insert(vp.rule(),"VP");
  rule_names.insert(s.rule(),"S");
  rule_names.insert(nn.rule(),"NN");
  rule_names.insert(nns.rule(),"NNS");
  rule_names.insert(vbz.rule(),"VBZ");
  rule_names.insert(dt.rule(),"DT");
  let mut parser = Parser::with_grammar(g.unwrap());
  Ok((parser, b, rule_names))
}

struct ExhaustiveTraverser {
  rule_names: HashMap<i32, &'static str>
}
struct PruningTraverser {
  rule_names: HashMap<i32, &'static str>
}


impl Traverser for ExhaustiveTraverser {
  type ParseTree = ();
  type ParseState = ();
  fn traverse_glade(&self, glade: &mut Glade, state: Self::ParseState) -> Result<(Self::ParseTree, Self::ParseState)> {
    // This routine converts the glade into a list of Penn-tagged elements.
    // It is called recursively.
    let rule_id = dbg!(glade.rule_id());
    let symbol_id = dbg!(glade.symbol_id());

    // A token is a single choice, and we know enough to fully Penn-tag it
    if rule_id == 0 {
    //   let literal  = glade.literal();
    //   let penn_tag = penn_tag.get(symbol_id);
    //   return Ok(vec![format!("({} {})",penn_tag, literal)]);
    }

    // let mut return_value = Vec::new();

    // loop {
    //   // The results at each position are a list of choices, so
    //   // to produce a new result list, we need to take a Cartesian
    //   // product of all the choices
    //   let mut results = vec![Vec::new()];
    //   for rh_ix in 0 .. glade.rh_length() {
    //     let mut new_results = Vec::new();
    //     for prev_result in results.drain(..) {
    //       let child_value = glade.rh_value(rh_ix);
    //       for new_value in child_value.into_iter() {
    //         let prev_update = prev_result.clone();
    //         prev_update.push(new_value);
    //         new_results.push(prev_update);
    //       }
    //     }
    //     results = new_results;
    //   }

    //   // Special case for the start rule
    //   // if ( $symbol_name eq '[:start]' ) {
    //   //   return [ map { join q{}, @{$_} } @results ];
    //   // }

    //   // Now we have a list of choices, as a list of lists.  Each sub list
    //   // is a list of Penn-tagged elements, which we need to join into
    //   // a single Penn-tagged element.  The result will be to collapse
    //   // one level of lists, and leave us with a list of Penn-tagged
    //   // elements

    //   return_value.push(results.into_iter().map(|result|
    //      format!("({} {})", penn_tag.get(symbol_id), result.join(" "))
    //   ));

    //   // Look at the next alternative in this glade, or end the
    //   // loop if there is none
    //   if glade.next().is_none() {
    //     break;
    //   }
    // }

    Ok(((),()))
  }
}

impl Traverser for PruningTraverser {
  type ParseTree = ();
  type ParseState = ();
  fn traverse_glade(&self, glade: &mut Glade, state: Self::ParseState) -> Result<(Self::ParseTree, Self::ParseState)> {
    Ok(((),()))
  }
}