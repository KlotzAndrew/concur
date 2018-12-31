extern crate uuid;
use uuid::Uuid;
use std::collections::HashMap;

pub struct GCounter {
  id: String,
  state: HashMap<String, i64>
}

impl GCounter {
  pub fn new() -> Self {
    let id = Uuid::new_v4().to_string();
    let mut state = HashMap::new();
    state.insert(id.clone(), 0);

    GCounter{id: id, state: state}
  }

  pub fn value(&self) -> i64 {
    self.state.values().sum()
  }

  pub fn merge(&mut self, other: GCounter) {
    for (id, value) in other.state {
      let current_value = self.state.entry(id).or_insert(0);

      if value > *current_value {
        *current_value = value;
      }
    }
  }

  pub fn inc(&mut self) {
    self.inc_by(1)
  }

  pub fn increment(&mut self, value: i64) {
    self.inc_by(value)
  }

  fn inc_by(&mut self, value: i64) {
     let id = &self.id;
    *self.state.entry(id.to_string()).or_insert(0) += value;
  }
}
