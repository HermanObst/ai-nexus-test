#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_push_and_pop() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<String> = Stack::new();
        stack.push("hello".to_string());
        stack.push("world".to_string());

        assert_eq!(stack.peek(), Some(&"world".to_string()));
        stack.pop();
        assert_eq!(stack.peek(), Some(&"hello".to_string()));
        stack.pop();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_is_empty() {
        let mut stack: Stack<bool> = Stack::new();
        assert_eq!(stack.is_empty(), true);
        stack.push(true);
        assert_eq!(stack.is_empty(), false);
        stack.pop();
        assert_eq!(stack.is_empty(), true);
    }
}
