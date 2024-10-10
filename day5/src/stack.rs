#[derive(Debug, PartialEq, Clone)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn build(items: Vec<T>) -> Self {
        Stack { stack: items }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
