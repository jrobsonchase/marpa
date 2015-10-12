use thin::Step;
use thin::Value;
use lexer::token::Token;

pub mod engine;

use self::engine::Engine;

struct Stack<T, U> where U: Engine<T>, T: Default + Clone {
    items: Vec<T>,
    engine: U,
}

impl<T, U> Stack<T, U> where U: Engine<T>, T: Default + Clone {
    fn new(engine: U) -> Stack<T, U> {
        let items = vec![Default::default(); 1];
        Stack{ items: items, engine: engine }
    }

    fn step(&mut self, value_step: Step) {
        match value_step {
            Step::Rule(rule, start, end) => {
                self.size_stack(end as usize);
                self.items[start as usize] = self.engine.proc_rule(rule, &self.items[start as usize..end as usize + 1]);
            },
            Step::Token(sym, res, val) => {
                self.size_stack(res as usize);
                self.items[res as usize] = self.engine.proc_token(Token::new(sym, val));
            },
            Step::NullingSymbol(sym, res) => {
                self.size_stack(res as usize);
                self.items[res as usize] = self.engine.proc_null(sym);
            },
            s => panic!("Invalid step: {:?}", s),
        }
    }

    fn size_stack(&mut self, last: usize) {
        let size = last + 1;
        let len = self.items.len();
        if len < size {
            self.items.reserve(size - len);
            for _ in (len..size) {
                self.items.push(Default::default());
            }
        }
    }

    fn proc_value(&mut self, val: &mut Value) -> &T {
        for v in val {
            self.step(v);
        }
        &self.items[0]
    }
}

pub fn proc_value<T: Default + Clone, U: Engine<T>>(eng: U, mut val: Value) -> T {
    let mut stack = Stack::new(eng);
    stack.proc_value(&mut val).clone()
}
