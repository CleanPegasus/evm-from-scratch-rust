

pub struct Memory {
  memory: Vec<u8> 
}


impl Memory {
  pub fn new() -> Self {
    let memory = Vec::new();
    Self {
      memory
    }
  }

  /// fetch arbitrary sized data
  pub fn access(&self, access: usize, size: usize) -> Vec<u8> {
    self.memory[access..access+size].to_vec()

  }

  /// fetch 32 byte data at offset
  pub fn load(&self, offset: usize) -> Vec<u8> {
    self.access(offset, 32)
  }

  /// Store memory and return the gas cost
  pub fn store(&mut self, offset: usize, value: Vec<u8>) -> usize {
    let mut memory_expansion_cost = 0;

    if self.memory.len() <= offset + value.len() {
      let mut expansion_size = 0;
      if self.memory.len() == 0 {
        expansion_size = 32;
        self.memory = vec![0x00; 32];
      }
      if self.memory.len() < offset + value.len() {
        expansion_size += offset + value.len() - self.memory.len();
        let new_size = self.memory.len() + expansion_size;
        self.memory.resize(new_size, 0x00);
      }

      memory_expansion_cost = expansion_size.pow(2);
    }

    self.memory[offset..value.len()].copy_from_slice(&value);
    
    memory_expansion_cost

  }
}