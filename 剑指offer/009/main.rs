struct CQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {

    fn new() -> Self {
        CQueue {
            stack1 : Vec::new(),
            stack2 : Vec::new(),
        }
    }
    
    fn append_tail(&mut self, value: i32) {
        self.stack1.push(value);
    }
    
    fn delete_head(&mut self) -> i32 {
        if self.stack2.len() != 0 {
            return self.stack2.pop().unwrap();
        }
        while self.stack1.len() != 0 {
            let num = self.stack1.pop().unwrap();
            self.stack2.push(num);
        }
        if self.stack2.len() != 0 {
            return self.stack2.pop().unwrap();
        }else {
            return -1;
        }
        /*match self.stack2.pop() {
            Some(n) => {
                return n;
            },
            None => {
                while !self.stack1.is_empty() {
                    let num = self.stack1.pop().unwrap();
                    self.stack2.push(num);
                }
                match self.stack2.pop() {
                    Some(n) => {
                        return n;
                    },
                    None => {
                        return -1;
                    }
                }
            }
        }*/
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */