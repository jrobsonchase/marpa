use thin::libmarpa_sys::*;

use thin::grammar;
use thin::grammar::Grammar;
use thin::symbol::Symbol;

use std::ops::Range;

#[derive(Debug)]
pub enum Event {
    None,
    CountedNullable(Symbol),
    EarleyItemThreshold(i32),
    Exhausted,
    LoopRules(i32),
    NullingTerminal(Symbol),
    SymbolCompleted(Symbol),
    SymbolExpected(Symbol),
    SymbolNulled(Symbol),
    SymbolPredicted(Symbol),
}

pub struct EventIter(Range<i32>, Grammar);

impl EventIter {
    pub fn new(count: i32, grammar: Grammar) -> EventIter {
        EventIter(0..count, grammar)
    }
}

impl Iterator for EventIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let mut event = Struct_marpa_event::default();
        match self.0.next() {
            None => None,
            Some(ix) => unsafe {
                match marpa_g_event(grammar::internal(&self.1), &mut event as *mut Struct_marpa_event, ix) {
                    MARPA_EVENT_NONE => Some(Event::None),
                    MARPA_EVENT_COUNTED_NULLABLE => Some(Event::CountedNullable(event.t_value)),
                    MARPA_EVENT_EARLEY_ITEM_THRESHOLD => Some(Event::EarleyItemThreshold(event.t_value)),
                    MARPA_EVENT_EXHAUSTED => Some(Event::Exhausted),
                    MARPA_EVENT_LOOP_RULES => Some(Event::LoopRules(event.t_value)),
                    MARPA_EVENT_NULLING_TERMINAL => Some(Event::NullingTerminal(event.t_value)),
                    MARPA_EVENT_SYMBOL_COMPLETED => Some(Event::SymbolCompleted(event.t_value)),
                    MARPA_EVENT_SYMBOL_EXPECTED => Some(Event::SymbolExpected(event.t_value)),
                    MARPA_EVENT_SYMBOL_NULLED => Some(Event::SymbolNulled(event.t_value)),
                    MARPA_EVENT_SYMBOL_PREDICTED => Some(Event::SymbolPredicted(event.t_value)),
                    ev => panic!("unknown event type: {}", ev),
                }
            },
        }
    }
}
