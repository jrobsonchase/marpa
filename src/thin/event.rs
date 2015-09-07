use thin::libmarpa_sys::*;
use thin::libc;

use thin::symbol::Symbol;
use thin::grammar;
use thin::grammar::Grammar;

use std::ops::Range;
use std::mem;


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

pub struct EventIter(Range<i32>, Grammar, *mut Struct_marpa_event);

impl EventIter {
    pub fn new(count: i32, grammar: Grammar) -> EventIter {
        unsafe {
            let event_struct = libc::malloc(mem::size_of::<Struct_marpa_event>() as libc::size_t) as *mut Struct_marpa_event;
            EventIter((0..count), grammar, event_struct)
        }
    }
}

impl Drop for EventIter {
    fn drop(&mut self) {
        unsafe { libc::free(self.2 as *mut libc::c_void) }
    }
}

impl Iterator for EventIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(ix) => {
                unsafe {
                    match marpa_g_event(grammar::internal(&self.1), self.2, ix) {
                        MARPA_EVENT_NONE => Some(Event::None),
                        MARPA_EVENT_COUNTED_NULLABLE => Some(Event::CountedNullable((*self.2).t_value)),
                        MARPA_EVENT_EARLEY_ITEM_THRESHOLD => Some(Event::EarleyItemThreshold((*self.2).t_value)),
                        MARPA_EVENT_EXHAUSTED => Some(Event::Exhausted),
                        MARPA_EVENT_LOOP_RULES => Some(Event::LoopRules((*self.2).t_value)),
                        MARPA_EVENT_NULLING_TERMINAL => Some(Event::NullingTerminal((*self.2).t_value)),
                        MARPA_EVENT_SYMBOL_COMPLETED => Some(Event::SymbolCompleted((*self.2).t_value)),
                        MARPA_EVENT_SYMBOL_EXPECTED => Some(Event::SymbolExpected((*self.2).t_value)),
                        MARPA_EVENT_SYMBOL_NULLED => Some(Event::SymbolNulled((*self.2).t_value)),
                        MARPA_EVENT_SYMBOL_PREDICTED => Some(Event::SymbolPredicted((*self.2).t_value)),
                        ev => panic!("unknown event type: {}", ev)
                    }
                }
            }
        }
    }
}
