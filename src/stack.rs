use std::fmt::{Display, Formatter};
use primitive_types::U256;

static MAX_STACK_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Stack {
    items: Vec<U256>
}

impl Stack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Push value to the top of the stack
    pub fn push(&mut self, value: U256) -> Result<(), &'static str> {
        if self.items.len() < MAX_STACK_SIZE {
            self.items.push(value);
            Ok(())
        } else {
            Err("Stack overflow")
        }
    }

    /// Pop value from the top of the stack
    pub fn pop(&mut self) -> Result<U256, &'static str> {
        self.items.pop().ok_or("Stack Underflow")
    }

    /// stack size
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Peek the stack
    pub fn peek(&self) -> Result<&U256, &'static str> {
        self.items.last().ok_or("Stack Empty")
    }
}

/// Display the stack
impl Display for Stack {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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