use crate::errors::CoodinationError;

pub struct LWWRegister {
  pub data: u32,
  version: u32
}

impl LWWRegister {
  pub fn new() -> Self {
    LWWRegister{data: 0, version: 0}
  }

  pub fn assign(&mut self, d: u32, v: u32) -> Result<(), CoodinationError> {
    if self.version < v {
      self.version = v;
      self.data = d;
      Ok(())
    } else if self.version == v && self.data != d {
      Err(CoodinationError::new("foo"))
    } else {
      Ok(())
    }
  }
}
