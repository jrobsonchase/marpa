use crate::thin::Step;
use crate::thin::Value;

pub mod processor;

use self::processor::Processor;

struct Stack<T>
where
    T: Processor,
{
    items: Vec<T::Tree>,
    processor: T,
}

impl<T> Stack<T>
where
    T: Processor,
{
    fn new(processor: T) -> Stack<T> {
        let items = vec![Default::default(); 1];
        Stack { items, processor }
    }

    fn step(&mut self, value_step: Step) {
        match value_step {
            Step::Rule(rule, start, end) => {
                self.size_stack(end as usize);
                self.items[start as usize] = self.processor.proc_rule(rule, &self.items[start as usize..=end as usize]);
            }
            Step::Token(sym, res, val) => {
                self.size_stack(res as usize);
                self.items[res as usize] = self.processor.proc_token((sym, val).into());
            }
            Step::NullingSymbol(sym, res) => {
                self.size_stack(res as usize);
                self.items[res as usize] = self.processor.proc_null(sym);
            }
            s => panic!("Invalid step: {:?}", s),
        }
    }

    fn size_stack(&mut self, last: usize) {
        let size = last + 1;
        let len = self.items.len();
        if len < size {
            self.items.reserve(size - len);
            for _ in len..size {
                self.items.push(Default::default());
            }
        }
    }

    fn proc_value(&mut self, val: &mut Value) -> &T::Tree {
        for v in val {
            self.step(v);
        }
        &self.items[0]
    }
}

pub fn proc_value<T: Processor>(eng: T, mut val: Value) -> T::Tree {
    let mut stack = Stack::new(eng);
    stack.proc_value(&mut val).clone()
}
