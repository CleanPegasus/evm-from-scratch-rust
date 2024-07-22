use std::fmt::{Display, Result, Formatter};

static MAX_STACK_SIZE: usize = 1024;
struct Stack {
    items: Vec<u8> // TODO: Create a custom hex type
}
  
impl Stack {
    fn new() -> Self {
        let items: Vec<u8> = Vec::new();
        Self {
            items
        }
    }

    fn push(&mut self, value: u8) {
        assert!(self.items.len() <= MAX_STACK_SIZE - 1, "Stack Overflow");
        self.items.push(value);
    }

    fn pop(&mut self, n: usize) {
        assert!(self.items.len() > n, "Stack Underflow");
        (0..n).map(|_| self.items.pop());
    }
}


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