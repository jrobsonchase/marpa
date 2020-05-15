
#[derive(Debug, Clone)]
pub struct Nidset {
  pub(crate) nids: Vec<i32>,
  pub(crate) id: usize
}

impl Nidset {
  pub fn get_nid(&self, index: usize) -> i32 {
    self.nids[index]
  }
}