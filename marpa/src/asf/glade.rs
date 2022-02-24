


#[derive(Debug, Clone)]
pub struct Glade {
  pub(crate) id: usize,
  pub(crate) symbol_id: i32,
  pub(crate) registered: bool,
  pub(crate) visited: bool,
  pub(crate) symches: Vec<usize>,
}

impl Default for Glade {
  fn default() -> Self {
    Glade {
      id: 0,
      symbol_id: -1,
      registered: false,
      visited: false,
      symches: Vec::new()
    }
  }
}

impl Glade {
  pub fn rule_id(&self) -> usize {
    self.id
  }

  pub fn symbol_id(&self) -> i32 {
    self.symbol_id
  }
}