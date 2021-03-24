const MAX_STACK_SIZE : usize = 1 << 16;
const STACK_EMPTY : usize = MAX_STACK_SIZE + 666;

struct Stack {
    vals : [i32; MAX_STACK_SIZE],
    top : usize,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            vals: [0; MAX_STACK_SIZE],
            top: STACK_EMPTY,
        }
    }

    fn clear(&mut self) {
        self.vals = [0; MAX_STACK_SIZE];
        self.top = STACK_EMPTY;
    }

    fn empty(&self) -> bool {
        self.top == STACK_EMPTY
    }

    fn push(&mut self, a : i32) -> bool {
        if self.empty() {
            self.top = 0;
        } else {
            self.top += 1;
        }
        if self.top >= MAX_STACK_SIZE {
            return false;
        } 
        self.vals[self.top] = a;
        true
    }
    
    fn peek(&mut self) -> Option<i32> {
        if self.empty() {
            return None;
        }
        Some(self.vals[self.top])
    }

    fn pop(&mut self) -> Option<i32> {
        let val = self.peek();
        if let Some(_) = val {
            if self.top == 0 {
                self.top = STACK_EMPTY;
            } else {
                self. top -= 1;
            }
        }
        val
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_clear() {
        let mut stk = Stack::new();
        assert_eq!(stk.vals, [0; MAX_STACK_SIZE]);
        assert_eq!(stk.top, STACK_EMPTY);
        stk.vals[0] = 666;
        stk.top = 666;
        stk.clear();
        assert_eq!(stk.vals, [0; MAX_STACK_SIZE]);
        assert_eq!(stk.top, STACK_EMPTY);
    }

    #[test]
    fn test_push_pop_peek() {
        let mut stk = Stack::new();
        assert!(stk.empty());
        
        // Push / Peek
        let top = stk.peek();
        assert!(top.is_none());

        assert!(stk.push(3));
        assert!(!stk.empty());
        let top = stk.peek();
        assert!(!top.is_none());
        assert_eq!(top.unwrap(), 3); 

        assert!(stk.push(4));
        assert!(!stk.empty());
        let top = stk.peek();
        assert!(!top.is_none());
        assert_eq!(top.unwrap(), 4); 

        assert!(stk.push(5));
        assert!(!stk.empty());
        let top = stk.peek();
        assert!(!top.is_none());
        assert_eq!(top.unwrap(), 5); 

        // Pop tests
        let pop_val = stk.pop();
        assert!(!pop_val.is_none());
        assert_eq!(pop_val.unwrap(), 5);

        let pop_val = stk.pop();
        assert!(!pop_val.is_none());
        assert_eq!(pop_val.unwrap(), 4);

        let pop_val = stk.pop();
        assert!(!pop_val.is_none());
        assert_eq!(pop_val.unwrap(), 3);

        let pop_val = stk.pop();
        assert!(pop_val.is_none());
        assert!(stk.empty());
    }
}
