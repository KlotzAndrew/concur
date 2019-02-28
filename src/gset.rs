use std::collections::HashSet;

#[derive(Clone)]
pub struct GSet {
  state: HashSet<String>
}

impl Default for GSet {
  fn default() -> Self {
    GSet::new()
  }
}

impl GSet {
  pub fn new() -> Self {
    GSet{state: HashSet::new()}
  }

  pub fn value(&self) -> HashSet<String> {
    self.state.clone()
  }

  pub fn merge(&mut self, other: &GSet) {
    self.state.extend(other.value());
  }

  pub fn insert(&mut self, element: String) {
    self.state.insert(element);
  }

  pub fn extend(&mut self, other: GSet) {
    self.state.extend(other.state);
  }
}
