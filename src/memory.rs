

struct Memory {
  memory: Vec<i32> 
}

impl Memory {
  fn new() -> Self {
    let memory = Vec::new();
    Self {
      memory
    }
  }

  fn access(&self, access: usize, size: usize) -> i32 {
    unimplemented!()
  }

  fn load(&self, offset: usize) -> i32 {
    unimplemented!()
  }

  fn store(&mut self, offset: usize, value: i32) {
    unimplemented!()
  }
}