use crate::thin::earley::*;
use crate::thin::event::EventIter;
use crate::thin::grammar as g;
use crate::thin::grammar::Grammar;
use crate::thin::progress::*;
use crate::thin::symbol::Symbol;
use crate::thin::order::Order;
use libmarpa_sys::*;

use crate::result::*;

use std::char;
use std::mem;

pub struct Recognizer {
    internal: Marpa_Recognizer,
    // we need to keep a reference to this accessible
    // in order to read error codes.
    grammar: Grammar,
}

result_from!(Recognizer, Grammar);

pub fn internal(recognizer: &Recognizer) -> Marpa_Recognizer {
    recognizer.internal
}

pub fn grammar(recognizer: &Recognizer) -> Grammar {
    recognizer.grammar.clone()
}

impl Clone for Recognizer {
    fn clone(&self) -> Recognizer {
        unsafe { marpa_r_ref(self.internal) };
        Recognizer {
            internal: self.internal,
            grammar: self.grammar.clone(),
        }
    }
}

impl Drop for Recognizer {
    fn drop(&mut self) {
        unsafe {
            marpa_r_unref(self.internal);
        }
    }
}

impl Recognizer {
    pub fn new(g: Grammar) -> Result<Recognizer> {
        let grammar = g::internal(&g);
        match unsafe { marpa_r_new(grammar) } {
            n if n.is_null() => g.error_or("error creating recognizer"),
            r => Ok(Recognizer { internal: r, grammar: g }),
        }
    }

    pub fn start_input(&mut self) -> Result<()> {
        match unsafe { marpa_r_start_input(self.internal) } {
            -2 => self.grammar.error_or("error starting input"),
            i if i >= 0 => Ok(()),
            i => panic!("unexpected error code: {}", i),
        }
    }

    pub fn alternative(&mut self, tok: Symbol, val: i32, len: i32) -> Result<()> {
        match unsafe { marpa_r_alternative(self.internal, tok, val, len) as _ } {
            MARPA_ERR_NONE => Ok(()),
            err => err_code(err),
        }
    }

    pub fn earleme_complete(&mut self) -> Result<i32> {
        match unsafe { marpa_r_earleme_complete(self.internal) } {
            -2 => self.grammar.error_or("error completing earleme"),
            evs if evs >= 0 => Ok(evs),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn current_earleme(&self) -> Result<Earleme> {
        match unsafe { marpa_r_current_earleme(self.internal) } {
            -1 => err_rnotstarted(),
            e if e >= 0 => Ok(e),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn furthest_earleme(&self) -> Result<Earleme> {
        Ok(unsafe { marpa_r_furthest_earleme(self.internal) } as Earleme)
    }

    pub fn earleme(&self, set: EarleySet) -> Result<Earleme> {
        match unsafe { marpa_r_earleme(self.internal, set) } {
            -2 => self.grammar.error_or("error getting earleme"),
            e if e >= 0 => Ok(e),
            err => panic!("unexpected error code: {}", err),
        }
    }

    pub fn latest_earley_set(&self) -> Result<EarleySet> {
        Ok(unsafe { marpa_r_latest_earley_set(self.internal) })
    }

    pub fn earley_set_value(&self, set: EarleySet) -> Result<char> {
        match unsafe { marpa_r_earley_set_value(self.internal, set) } {
            -2 => self.grammar.error_or("error getting set value"),
            val => Ok(char::from_u32(val as u32).unwrap()),
        }
    }

    pub fn latest_earley_set_value_set(&self, c: char) -> Result<()> {
        match unsafe { marpa_r_latest_earley_set_value_set(self.internal, c as i32) } {
            -2 => self.grammar.error_or("error setting set value"),
            _ => Ok(()),
        }
    }

    pub fn completion_symbol_activate(&mut self, sym: Symbol, reactivate: bool) -> Result<()> {
        match unsafe { marpa_r_completion_symbol_activate(self.internal, sym, reactivate as i32) } {
            -2 => self.grammar.error_or("error setting symbol activation"),
            _ => Ok(()),
        }
    }

    pub fn earley_item_warning_threshold_set(&mut self, thresh: i32) {
        unsafe { marpa_r_earley_item_warning_threshold_set(self.internal, thresh) };
    }

    pub fn earley_item_warning_threshold(&self) -> i32 {
        unsafe { marpa_r_earley_item_warning_threshold(self.internal) }
    }

    pub fn expected_symbol_event_set(&mut self, sym: Symbol, expected: bool) -> Result<()> {
        match unsafe { marpa_r_expected_symbol_event_set(self.internal, sym, expected as i32) } {
            -2 => self.grammar.error_or("error setting symbol expected event"),
            _ => Ok(()),
        }
    }

    pub fn is_exhausted(&self) -> bool {
        match unsafe { marpa_r_is_exhausted(self.internal) } {
            1 => true,
            0 => false,
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn nulled_symbol_activate(&mut self, sym: Symbol, activate: bool) -> Result<()> {
        match unsafe { marpa_r_nulled_symbol_activate(self.internal, sym, activate as i32) } {
            -2 => self.grammar.error_or("error setting nulled symbol activate"),
            _ => Ok(()),
        }
    }

    pub fn terminals_expected(&self) -> Result<Vec<Symbol>> {
        let syms = self.grammar.num_symbols()? as usize;
        let mut tmp: Vec<Symbol> = Vec::with_capacity(syms);
        match unsafe { marpa_r_terminals_expected(self.internal, tmp.as_mut_ptr()) } {
            -2 => self.grammar.error_or("error getting expected terminals"),
            i if i >= 0 => {
                let data_ptr = tmp.as_mut_ptr();
                mem::forget(tmp);
                unsafe { Ok(Vec::from_raw_parts(data_ptr, i as usize, syms)) }
            }
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn terminal_is_expected(&self, sym: Symbol) -> Result<bool> {
        match unsafe { marpa_r_terminal_is_expected(self.internal, sym) } {
            -2 => self.grammar.error_or("error getting terminal is_expected"),
            0 => Ok(false),
            i if i > 0 => Ok(true),
            e => panic!("unexpected error code: {}", e),
        }
    }

    #[allow(dead_code)]
    fn progress_report_reset(&mut self) -> Result<()> {
        match unsafe { marpa_r_progress_report_reset(self.internal) } {
            -2 => self.grammar.error_or("error resetting progress report"),
            i if i >= 0 => Ok(()),
            e => panic!("unexpected error code: {}", e),
        }
    }

    fn progress_report_start(&self, set: EarleySet) -> Result<i32> {
        match unsafe { marpa_r_progress_report_start(self.internal, set) } {
            -2 => self.grammar.error_or("error starting progress report"),
            n if n >= 0 => Ok(n),
            e => panic!("unexpected error code: {}", e),
        }
    }

    fn progress_report_finish(&self) -> Result<()> {
        match unsafe { marpa_r_progress_report_finish(self.internal) } {
            -2 => self.grammar.error_or("error finishing progress report"),
            n if n >= 0 => Ok(()),
            e => panic!("unexpected error code: {}", e),
        }
    }

    fn progress_report_item(&self) -> Result<ProgressItem> {
        let mut pos: i32 = 0;
        let mut origin: EarleySet = 0;
        match unsafe { marpa_r_progress_item(self.internal, &mut pos as *mut i32, &mut origin as *mut EarleySet) } {
            -2 => self.grammar.error_or("error getting next progress report item"),
            -1 => err_code(MARPA_ERR_PROGRESS_REPORT_EXHAUSTED),
            ruleid if ruleid >= 0 => Ok(ProgressItem { rule: ruleid, pos, origin }),
            e => panic!("unexpected error code: {}", e),
        }
    }

    pub fn progress_report(&self, set: EarleySet) -> Result<ProgressReport> {
        let num_items = self.progress_report_start(set)?;
        let mut report: Vec<ProgressItem> = Vec::new();
        let mut res: Option<Result<ProgressReport>> = None;
        for _ in 0..num_items {
            match self.progress_report_item() {
                Ok(item) => report.push(item),
                Err(err) => {
                    res = Some(Err(err));
                    break;
                }
            }
        }

        self.progress_report_finish()?;

        if res.is_none() {
            res = Some(Ok(report));
        }

        res.unwrap()
    }

    pub fn events(&self) -> Result<EventIter> {
        self.grammar.events()
    }

    pub fn grammar(&self) -> &Grammar {
        &self.grammar
    }

    pub fn ordering_get(&self) -> Option<Order> {
        None
        // if recce.no_parse {
        //     return None;
        // }
        // if let Some(ordering) = self.ordering {
        //     return Some(ordering);
        // }
        // let parse_set_arg =self.end_of_parse;
        // let grammar_c   = self.grammar.internal();
        // let recce_c   = self.internal;
        // $grammar_c->throw_set(0);
        // my $bocage = $recce->[Marpa::R2::Internal::Recognizer::B_C] =
        //     Marpa::R2::Thin::B->new( $recce_c, ( $parse_set_arg // -1 ) );
        // $grammar_c->throw_set(1);
        // if ( not $bocage ) {
        //     $recce->[Marpa::R2::Internal::Recognizer::NO_PARSE] = 1;
        //     return;
        // }
        // $ordering = $recce->[Marpa::R2::Internal::Recognizer::O_C] =
        //     Marpa::R2::Thin::O->new($bocage);

        // GIVEN_RANKING_METHOD: {
        //     my $ranking_method =
        //         $recce->[Marpa::R2::Internal::Recognizer::RANKING_METHOD];
        //     if ( $ranking_method eq 'high_rule_only' ) {
        //         do_high_rule_only($recce);
        //         last GIVEN_RANKING_METHOD;
        //     }
        //     if ( $ranking_method eq 'rule' ) {
        //         do_rank_by_rule($recce);
        //         last GIVEN_RANKING_METHOD;
        //     }
        // } ## end GIVEN_RANKING_METHOD:

        // ordering
    }
}

#[cfg(test)]
mod tests {
    use crate::thin::event::Event;
    use crate::thin::grammar::Grammar;
    use crate::thin::recognizer::Recognizer;

    #[test]
    fn create_recognizer() {
        let mut g: Grammar = Grammar::new().unwrap();
        let start = g.new_symbol().unwrap();
        g.set_start_symbol(start).unwrap();
        g.new_rule(start, &[]).unwrap();
        g.precompute().unwrap();
        assert!(g.symbol_is_nulling(start).unwrap());
        assert!(g.events().unwrap().collect::<Vec<Event>>().len() == 0);

        let mut r: Recognizer = Recognizer::new(g).unwrap();

        r.start_input().unwrap();

        let evs: Vec<Event> = r.events().unwrap().collect();
        for e in evs.iter() {
            println!("Event: {:?}", e);
        }
        assert!(evs.len() != 0);
    }
}
