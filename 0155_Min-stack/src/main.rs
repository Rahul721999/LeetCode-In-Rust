struct Minstack {
    arr: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Minstack {
    fn new() -> Self {
        Minstack { arr: vec![] }
    }

    fn push(&mut self, val: i32) {
        let min = if self.arr.is_empty() {
            val
        } else {
            std::cmp::min(val, self.get_min())
        };
        self.arr.push((val, min));
    }

    fn pop(&mut self) {
        if !self.arr.is_empty() {
            let _ = self.arr.pop();
        }
    }

    fn top(&self) -> i32 {
        if let Some(&(val, _)) = self.arr.last() {
            return val;
        }
        panic!("Stack is empty")
    }

    fn get_min(&self) -> i32 {
        if let Some(&(_, min)) = self.arr.last() {
            return min;
        }
        panic!("Stack is empty")
    }
}

fn main() {
    let mut min_stack = Minstack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(-3, min_stack.get_min()); // return -3
    min_stack.pop();
    assert_eq!(0, min_stack.top()); // return 0
    assert_eq!(-2, min_stack.get_min()); // return -2
}
