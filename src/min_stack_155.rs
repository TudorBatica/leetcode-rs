// https://leetcode.com/problems/min-stack/

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        match self.min_stack.last() {
            None => { self.min_stack.push(val) }
            Some(last) => { if last >= &val { self.min_stack.push(val) } }
        }
    }

    fn pop(&mut self) {
        match self.stack.pop() {
            None => {}
            Some(popped) => {
                if self.min_stack.last().eq(&Some(&popped)) {
                    self.min_stack.pop();
                }
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().expect("Should be called only in valid situations as per the leetcode description")
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().expect("Should be called only in valid situations as per the leetcode description")
    }
}
