use std::fmt::{Display, Result, Formatter};

static MAX_STACK_SIZE: usize = 1024;
pub struct Stack {
    items: Vec<u8>
}
  
impl Stack {
    pub fn new() -> Self {
        let items: Vec<u8> = Vec::new();
        Self {
            items
        }
    }

    /// Push value to the top of the stack
    pub fn push(&mut self, value: u8) {
        assert!(self.items.len() <= MAX_STACK_SIZE - 1, "Stack Overflow");
        self.items.push(value);
    }

    /// Pop value from the top of the stack
    pub fn pop(&mut self, n: usize) -> Vec<u8> {
        assert!(self.items.len() > n, "Stack Underflow");
        let popped_values = (0..n).map(|_| self.items.pop().unwrap()).collect();
        popped_values
    }
}

/// Display the stack
impl Display for Stack {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      let len = self.items.len();
      for (index, value) in self.items.iter().rev().enumerate() {
          if index == 0 {
              write!(f, "{} <-- top", value)?;
          } else if index == len - 1 {
              write!(f, "{} <-- bottom", value)?;
          } else {
              write!(f, "{}", value)?;
          }
          if index < len - 1 {
              write!(f, ", ")?;
          }
      }
      Ok(())
  }
}