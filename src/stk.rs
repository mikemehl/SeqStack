/// Implementation of the different stacks used by our vm.

const MAX_STACK_SIZE: usize = 1 << 16;
const STACK_EMPTY: usize = MAX_STACK_SIZE + 666;

pub struct Stack {
    vals: [i32; MAX_STACK_SIZE],
    top: usize,
}

impl Copy for Stack {}
impl Clone for Stack {
    fn clone(&self) -> Stack {
        Stack {
            vals: self.vals,
            top: self.top,
        }
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            vals: [0; MAX_STACK_SIZE],
            top: STACK_EMPTY,
        }
    }

    pub fn clear(&mut self) {
        self.vals = [0; MAX_STACK_SIZE];
        self.top = STACK_EMPTY;
    }

    pub fn empty(&self) -> bool {
        self.top == STACK_EMPTY
    }

    pub fn full(&self) -> bool {
        self.top == (MAX_STACK_SIZE - 1)
    }

    pub fn push(&mut self, a: i32) -> bool {
        if self.empty() {
            self.top = 0;
        } else if self.full() {
            return false;
        } else {
            self.top += 1;
        }
        self.vals[self.top] = a;
        true
    }

    pub fn peek(&mut self) -> Option<i32> {
        if self.empty() {
            return None;
        }
        Some(self.vals[self.top])
    }

    pub fn pop(&mut self) -> Option<i32> {
        let val = self.peek();
        if val.is_some() {
            if self.top == 0 {
                self.top = STACK_EMPTY;
            } else {
                self.top -= 1;
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

        // Fill 'er up test.
        while !stk.full() {
            stk.push(666);
        }
        assert!(!stk.push(666));
    }
}
