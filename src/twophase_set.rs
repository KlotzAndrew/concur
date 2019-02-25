use std::collections::HashSet;

use crate::gset::GSet;

#[derive(Clone)]
pub struct TwoPhaseSet {
  add_set: GSet,
  rm_set: GSet
}

impl Default for TwoPhaseSet {
  fn default() -> Self {
    TwoPhaseSet::new()
  }
}

impl TwoPhaseSet {
  pub fn new() -> Self {
    TwoPhaseSet{add_set: GSet::new(), rm_set: GSet::new()}
  }

  pub fn value(&self) -> HashSet<String> {
    self.add_set.value().difference(&self.rm_set.value()).cloned().collect()
  }

  pub fn merge(&mut self, other: TwoPhaseSet) {
    self.add_set.extend(other.add_set);
    self.rm_set.extend(other.rm_set);
  }

  pub fn insert(&mut self, element: String) {
    self.add_set.insert(element);
  }

  pub fn remove(&mut self, element: String) {
    self.rm_set.insert(element);
  }
}
