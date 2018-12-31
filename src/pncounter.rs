extern crate uuid;
use uuid::Uuid;
use std::collections::HashMap;

pub struct PNCounter {
  id: String,
  pos: HashMap<String, i64>,
  neg: HashMap<String, i64>
}

impl PNCounter {
  pub fn new() -> Self {
    let id = Uuid::new_v4().to_string();
    let mut pos = HashMap::new();
    pos.insert(id.clone(), 0);

    let mut neg = HashMap::new();
    neg.insert(id.clone(), 0);

    PNCounter{id: id, pos: pos, neg: neg}
  }

  pub fn value(&self) -> i64 {
    let pos : i64 = self.pos.values().sum();
    let neg : i64 = self.neg.values().sum();
    pos - neg
  }

  pub fn merge(&mut self, other: PNCounter) {
    for (id, value) in other.pos {
      let current_value = self.pos.entry(id).or_insert(0);

      if value > *current_value {
        *current_value = value;
      }
    }

    for (id, value) in other.neg {
      let current_value = self.neg.entry(id).or_insert(0);

      if value > *current_value {
        *current_value = value;
      }
    }
  }

  pub fn dec(&mut self) {
    self.dec_by(1)
  }

  pub fn decrement(&mut self, value: i64) {
    self.dec_by(value)
  }

  fn dec_by(&mut self, value: i64) {
     let id = &self.id;
    *self.neg.entry(id.to_string()).or_insert(0) += value;
  }

  pub fn inc(&mut self) {
    self.inc_by(1)
  }

  pub fn increment(&mut self, value: i64) {
    self.inc_by(value)
  }

  fn inc_by(&mut self, value: i64) {
     let id = &self.id;
    *self.pos.entry(id.to_string()).or_insert(0) += value;
  }
}
