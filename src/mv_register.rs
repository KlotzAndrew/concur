use std::collections::HashMap;

#[derive(Clone)]
pub struct MVRegister {
  elements: HashMap<String, Element>
}

#[derive(Clone)]
struct Element {
  id: String,
  pub value: String,
  version: u32
}

impl Default for MVRegister {
  fn default() -> Self {
    MVRegister::new()
  }
}

impl MVRegister {
  pub fn new() -> Self {
    MVRegister{elements: HashMap::new()}
  }

  pub fn upsert(&mut self, id: String, value: String) {
    let element = self.elements.entry(id.clone()).or_insert(Element{id, value, version: 1});

    if element.version > 1 {
      element.version+=1;
    }
  }

  pub fn get(self, id: &str) -> String {
    match self.elements.get(id) {
      Some(element) => element.value.clone(),
      None => "".to_string()
    }
  }

  pub fn merge(&mut self, other: MVRegister) {
    for (id, element) in other.elements {
      let current_element = self.elements.entry(id).or_insert_with(|| element.clone());

      if element.version >= current_element.version {
        current_element.version = element.version
      }
    }
  }
}
