use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ORSet {
  pub elements: HashMap<String, Element>
}

impl Default for ORSet {
  fn default() -> Self {
    ORSet::new()
  }
}

#[derive(Debug, Clone)]
pub struct Element {
  version: u32,
  pub removed: bool
}

impl ORSet {
  pub fn new() -> Self {
    ORSet{elements: HashMap::new()}
  }

  pub fn add(&mut self, value: &str) {
    let element = self.elements.entry(value.to_string()).or_insert(Element{version: 0, removed: false});

    element.version+=1;
    element.removed = false;
  }

  pub fn remove(&mut self, value: &str) {
    let mut element = self.elements.entry(value.to_string()).or_insert(Element{version: 0, removed: true});

    element.version+=1;
    element.removed = true;
  }

  pub fn value(self) -> HashSet<String> {
    self.elements.clone().iter()
      .filter(|(_, v)| !v.removed )
      .map(|(k, _)| k.to_string())
      .collect::<HashSet<String>>()
      .clone()
  }

  pub fn merge(&mut self, other: &ORSet) {
    for (id, element) in other.elements.clone() {
      let current_element = self.elements.entry(id).or_insert_with(|| element.clone());

      if element.version >= current_element.version {
        current_element.version = element.version;
        current_element.removed = element.removed;
      }
    }
  }
}
