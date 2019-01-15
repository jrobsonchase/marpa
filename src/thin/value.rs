use thin::libmarpa_sys::*;
use thin::tree;
use thin::{Grammar, Result, Step, Tree};

pub struct Value {
    internal: Marpa_Value,
    // we need to keep a reference to this accessible
    // in order to read error codes.
    grammar: Grammar,
}

impl Value {
    pub fn new(t: Tree) -> Result<Value> {
        let t_internal = tree::internal(&t);
        let grammar = tree::grammar(&t);
        match unsafe { marpa_v_new(t_internal) } {
            n if n.is_null() => grammar.error_or("error creating value"),
            v => Ok(Value { internal: v, grammar }),
        }
    }
}

#[allow(dead_code)]
pub fn internal(value: &Value) -> Marpa_Value {
    value.internal
}

#[allow(dead_code)]
pub fn grammar(value: &Value) -> Grammar {
    value.grammar.clone()
}

impl Clone for Value {
    fn clone(&self) -> Value {
        unsafe { marpa_v_ref(self.internal) };
        Value {
            internal: self.internal,
            grammar: self.grammar.clone(),
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { marpa_v_unref(self.internal) };
    }
}

impl Iterator for Value {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match marpa_v_step(self.internal) {
                MARPA_STEP_INITIAL => self.next(),
                MARPA_STEP_INACTIVE => None,

                MARPA_STEP_NULLING_SYMBOL => Some(Step::NullingSymbol((*self.internal).t_token_id, (*self.internal).t_result)),

                MARPA_STEP_RULE => Some(Step::Rule(
                    (*self.internal).t_rule_id,
                    (*self.internal).t_result,
                    (*self.internal).t_arg_n,
                )),

                MARPA_STEP_TOKEN => Some(Step::Token(
                    (*self.internal).t_token_id,
                    (*self.internal).t_result,
                    (*self.internal).t_token_value,
                )),

                // the only thing left are the internal-only and invalid types
                _ => None,
            }
        }
    }
}
