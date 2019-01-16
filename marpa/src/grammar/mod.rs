use crate::result::Result;
use std::collections::HashMap;
use crate::thin;

pub struct Grammar {
    internal: thin::Grammar,
    rules: HashMap<thin::Rule, thin::Symbol>,
}

#[derive(Copy, Clone, Debug)]
pub enum Item {
    Rule(thin::Rule),
    Symbol(thin::Symbol),
}

impl Item {
    pub fn rule(self) -> thin::Rule {
        match self {
            Item::Rule(r) => r,
            Item::Symbol(_) => panic!("rule called on non-rule Item: {:?}", self),
        }
    }

    pub fn symbol(self) -> thin::Symbol {
        match self {
            Item::Symbol(s) => s,
            Item::Rule(_) => panic!("symbol called on non-symbol Item: {:?}", self),
        }
    }
}

impl Default for Grammar {
    fn default() -> Self {
        Grammar {
            internal: thin::Grammar::new().unwrap(),
            rules: Default::default(),
        }
    }
}

impl Grammar {
    pub fn new() -> Result<Self> {
        let mut g = Grammar {
            internal: thin::Grammar::new()?,
            ..Default::default()
        };

        for _ in 0..256 {
            g.internal.new_symbol()?;
        }

        Ok(g)
    }

    fn get_lhs(&mut self, lhs: Option<Item>) -> Result<thin::Symbol> {
        match lhs {
            Some(it) => Ok(self.symbol(it)),
            None => self.internal.new_symbol(),
        }
    }

    pub fn new_symbol(&mut self) -> Result<Item> {
        Ok(Item::Symbol(self.internal.new_symbol()?))
    }

    pub fn set_start(&mut self, it: Item) -> Result<Item> {
        let sym = self.symbol(it);
        self.internal.set_start_symbol(sym)?;
        Ok(it)
    }

    pub fn symbol(&self, item: Item) -> thin::Symbol {
        match item {
            Item::Rule(ref rule) => self.rules[rule],
            Item::Symbol(ref sym) => *sym,
        }
    }

    pub fn symbols(&self, items: &[Item]) -> Vec<thin::Symbol> {
        items.iter().map(|x| self.symbol(*x)).collect()
    }

    pub fn unwrap(self) -> thin::Grammar {
        self.internal
    }

    pub fn rule(&mut self, lhs: Option<Item>, rhs: &[Item]) -> Result<Item> {
        let lhs = self.get_lhs(lhs)?;
        let rhs = self.symbols(rhs);
        let r = self.internal.new_rule(lhs, &rhs)?;
        self.rules.insert(r, lhs);
        Ok(Item::Rule(r))
    }

    pub fn sequence(&mut self, lhs: Option<Item>, rhs: Item, sep: Item, nonempty: bool, proper: bool) -> Result<Item> {
        let lhs = self.get_lhs(lhs)?;
        let sep = self.symbol(sep);
        let rhs = self.symbol(rhs);
        let r = self.internal.new_sequence(lhs, rhs, sep, proper, nonempty)?;
        self.rules.insert(r, lhs);
        Ok(Item::Rule(r))
    }

    pub fn plus(&mut self, lhs: Option<Item>, rhs: Item) -> Result<Item> {
        let lhs = self.get_lhs(lhs)?;
        let internal = self.internal.new_symbol()?;
        let rec = self.internal.new_symbol()?;
        let rhs = self.symbol(rhs);
        self.internal.new_rule(internal, &[rhs])?;
        self.internal.new_rule(rec, &[internal])?;
        self.internal.new_rule(rec, &[rec, internal])?;
        let r = self.internal.new_rule(lhs, &[rec])?;
        self.rules.insert(r, lhs);
        Ok(Item::Rule(r))
    }

    pub fn star(&mut self, lhs: Option<Item>, rhs: Item) -> Result<Item> {
        let lhs = self.get_lhs(lhs)?;
        let internal = self.internal.new_symbol()?;
        let rec = self.internal.new_symbol()?;
        let rhs = self.symbol(rhs);
        self.internal.new_rule(internal, &[rhs])?;
        self.internal.new_rule(rec, &[])?;
        self.internal.new_rule(rec, &[rec, internal])?;
        let r = self.internal.new_rule(lhs, &[rec])?;
        self.rules.insert(r, lhs);
        Ok(Item::Rule(r))
    }

    pub fn maybe(&mut self, lhs: Option<Item>, rhs: Item) -> Result<Item> {
        let lhs = self.get_lhs(lhs)?;
        let rhs = self.symbol(rhs);
        let internal = self.internal.new_symbol()?;
        self.internal.new_rule(internal, &[])?;
        self.internal.new_rule(internal, &[rhs])?;
        let r = self.internal.new_rule(lhs, &[internal])?;
        self.rules.insert(r, lhs);
        Ok(Item::Rule(r))
    }

    pub fn alternative(&mut self, lhs: Option<Item>, rhs: &[Item]) -> Result<Item> {
        let lhs = self.get_lhs(lhs)?;
        let internal = self.internal.new_symbol()?;

        for it in rhs.iter() {
            let it = self.symbol(*it);
            self.internal.new_rule(internal, &[it])?;
        }

        let r = self.internal.new_rule(lhs, &[internal])?;
        self.rules.insert(r, lhs);
        Ok(Item::Rule(r))
    }

    pub fn literal_string<S: Into<String>>(&mut self, lhs: Option<Item>, input: S) -> Result<Item> {
        self.rule(lhs, &string_to_items(input))
    }

    // TODO optimize this
    pub fn byte_range(&mut self, lhs: Option<Item>, from: u8, to: u8) -> Result<Item> {
        self.alternative(lhs, &bytes_to_items(&(from..=to).collect::<Vec<u8>>()))
    }

    pub fn byte_set(&mut self, lhs: Option<Item>, bytes: &[u8]) -> Result<Item> {
        self.alternative(lhs, &bytes_to_items(bytes))
    }

    pub fn inverse_byte_set(&mut self, lhs: Option<Item>, bytes: &[u8]) -> Result<Item> {
        use std::collections::HashSet;
        let mut set: HashSet<u8> = HashSet::new();
        for b in bytes.iter() {
            set.insert(*b);
        }

        let lhs = self.get_lhs(lhs)?;
        let internal = self.internal.new_symbol()?;

        for b in (::std::ops::Range::<u16> { start: 0, end: 256 }) {
            if !set.contains(&(b as u8)) {
                self.internal.new_rule(internal, &[i32::from(b)])?;
            }
        }

        let r = self.internal.new_rule(lhs, &[internal])?;
        self.rules.insert(r, lhs);
        Ok(Item::Rule(r))
    }

    pub fn char_range(&mut self, lhs: Option<Item>, from: char, to: char) -> Result<Item> {
        self.byte_range(lhs, from as u8, to as u8)
    }

    pub fn string_set<S: Into<String>>(&mut self, lhs: Option<Item>, input: S) -> Result<Item> {
        self.byte_set(lhs, input.into().as_bytes())
    }

    pub fn inverse_string_set<S: Into<String>>(&mut self, lhs: Option<Item>, input: S) -> Result<Item> {
        self.inverse_byte_set(lhs, input.into().as_bytes())
    }
}

fn bytes_to_items(input: &[u8]) -> Vec<Item> {
    input.iter().map(|x| Item::Symbol(i32::from(*x))).collect()
}

fn string_to_items<S: Into<String>>(input: S) -> Vec<Item> {
    input.into().as_bytes().iter().map(|x| Item::Symbol(i32::from(*x))).collect()
}
