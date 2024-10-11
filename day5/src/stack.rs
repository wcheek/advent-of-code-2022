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

    pub fn push(&mut self, item: Vec<T>) {
        self.stack.push(item)
    }

    pub fn pop(&mut self, mut num_to_pop: usize) -> Option<Vec<T>> {
        let mut temp = vec![];

        while num_to_pop > 0 {
            let temp_crate = self.stack.pop();
            temp.push(temp_crate.unwrap());
            num_to_pop -= 1;
        }
        temp.reverse();
        Some(temp)
    }

    pub fn reverse(&mut self) {
        self.stack.reverse();
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    // fn length(&self) -> usize {
    //     self.stack.len()
    // }
    //
    // fn is_empty(&self) -> bool {
    //     self.stack.is_empty()
    // }
}
