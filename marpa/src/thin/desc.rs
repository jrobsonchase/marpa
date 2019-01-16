#![allow(unused)]

const MARPA_EVENT_DESCRIPTION: &'static [MarpaDescription] = &[
    (0, "MARPA_EVENT_NONE", "No event"),
    (1, "MARPA_EVENT_COUNTED_NULLABLE", "This symbol is a counted nullable"),
    (2, "MARPA_EVENT_EARLEY_ITEM_THRESHOLD", "Too many Earley items"),
    (3, "MARPA_EVENT_EXHAUSTED", "Recognizer is exhausted"),
    (4, "MARPA_EVENT_LOOP_RULES", "Grammar contains a infinite loop"),
    (5, "MARPA_EVENT_NULLING_TERMINAL", "This symbol is a nulling terminal"),
    (6, "MARPA_EVENT_SYMBOL_COMPLETED", "Completed symbol"),
    (7, "MARPA_EVENT_SYMBOL_EXPECTED", "Expecting symbol"),
    (8, "MARPA_EVENT_SYMBOL_NULLED", "Symbol was nulled"),
    (9, "MARPA_EVENT_SYMBOL_PREDICTED", "Symbol was predicted"),
];

const MARPA_STEP_TYPE_NAME: &'static [MarpaDescription] = &[
    (0, "MARPA_STEP_INTERNAL1", ""),
    (1, "MARPA_STEP_RULE", ""),
    (2, "MARPA_STEP_TOKEN", ""),
    (3, "MARPA_STEP_NULLING_SYMBOL", ""),
    (4, "MARPA_STEP_TRACE", ""),
    (5, "MARPA_STEP_INACTIVE", ""),
    (6, "MARPA_STEP_INTERNAL2", ""),
    (7, "MARPA_STEP_INITIAL", ""),
];

#[cfg(test)]
mod tests {
    use thin::desc;

    #[test]
    fn test_not_ok() {
        assert!(desc::err_desc(29) == "Marpa is in a not OK state");
    }
}
