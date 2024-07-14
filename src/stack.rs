static MAX_STACK_SIZE: usize = 1024;
struct Stack {
    items: Vec<i32> // TODO: Create a custom hex type
}
  
impl Stack {
    fn new() -> Self {
        let items: Vec<i32> = Vec::new();
        Self {
            items
        }
    }

    fn push(&mut self, value: i32) {
        assert!(self.items.len() <= MAX_STACK_SIZE - 1, "Stack Overflow");
        self.items.push(value);
    }

    fn pop(&mut self, n:i32) {
        assert!(self.items.len() > n as usize, "Stack Overflow");
        (0..n).map(|_| self.items.pop());
    }
}