// use std::collections::BinaryHeap;
// use std::cmp::Reverse;

// Note from personal wiki
// MinStack: One point I missed: the data is only accessed from the right!
// So in order to find out what item is minimum on the stack to the -left-
// of current point I just need to create another stack with minimum item
// of the left one... nice neetcode! Hence he works on google...
// Luckily that max-O(n) "amortized O(1) algorithm works just fine...
// (conditional min update)"

#[derive(Debug)]
// struct MinStack {
//     heap: BinaryHeap<Reverse<i32>>,
//     top: i32,
// }
// struct MinStack {
//     stack: Vec<i32>,
//     min: Option<i32>,
// }
// neetcode's nice solution
// struct MinStack {
//     stack: Vec<i32>,
//     min: Vec<i32>,
// }
// with reference
// annoying: better raw pointer for this case? anyway!
struct MinStack {
    stack: Vec<i32>,
    min: Vec<*const i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.min.push(match self.min.last() {
            Some(v) => {
                unsafe {
                    if val < **v {
                        // dangerous: I don't know if
                        // list get mutated if reallocated!
                        // quite an important lesson!
                        self.stack.last().unwrap() as *const i32
                    } else {
                        *self.min.last().unwrap()
                    }
                }
            }
            None => self.stack.last().unwrap() as *const i32,
        });
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().to_owned()
    }

    fn get_min(&mut self) -> i32 {
        unsafe { *self.min.last().unwrap().to_owned() }
    }
}
// #[allow(unused)]
// impl MinStack {
//     fn new() -> Self {
//         Self {
//             stack: Vec::new(),
//             min: Vec::new(),
//         }
//     }

//     fn push(&mut self, val: i32) {
//         self.stack.push(val);
//         self.min.push(match self.min.last() {
//             Some(v) => {
//                 if val < *v {
//                     val
//                 } else {
//                     *v
//                 }
//             },
//             None => val,
//         })
//     }

//     fn pop(&mut self) {
//         self.stack.pop();
//         self.min.pop();
//     }

//     fn top(&self) -> i32 {
//         self.stack.last().unwrap().to_owned()
//     }

//     fn get_min(&mut self) -> i32 {
//         self.min.last().unwrap().to_owned()
//     }
// }
// #[allow(unused)]
// impl MinStack {
//     fn new() -> Self {
//         Self {
//             stack: Vec::new(),
//             min: None,
//         }
//     }

//     fn push(&mut self, val: i32) {
//         self.stack.push(val);
//         self.min = match self.min {
//             Some(v) => Some(val.min(v)),
//             None => Some(val),
//         }
//     }

//     fn pop(&mut self) {
//         if let Some(v) = self.stack.pop() {
//             if let Some(v) = self.min {
//                 self.min = self.stack.iter().min().map(|x| x.to_owned());
//             }
//         }
//     }

//     fn top(&self) -> i32 {
//         self.stack.last().unwrap().to_owned()
//     }

//     fn get_min(&mut self) -> i32 {
//         self.min.unwrap()
//     }
// }

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

fn main() {}

#[test]
fn test_solution() {
    let mut k = MinStack::new();
    k.push(-2);
    k.push(-0);
    k.push(-3);
    assert_eq!(k.get_min(), -3);
    k.pop();
    assert_eq!(k.top(), 0);
    assert_eq!(k.get_min(), -2);

    // wtf unclear what it wants. poor wording all around
    k = MinStack::new();
    k.push(-2);
    k.push(0);
    k.push(-1);
    assert_eq!(k.get_min(), -2);
    assert_eq!(k.top(), -1);
    k.pop();
    assert_eq!(k.get_min(), -2);

    // hmm interesting edge case
    k = MinStack::new();
    k.push(2147483646);
    k.push(2147483646);
    k.push(2147483647);
    println!("{}", k.top());
    k.pop();
    println!("{}", k.get_min());
    k.pop();
    println!("{}", k.get_min());
    k.pop();
    k.push(2147483647);
    println!("{}", k.top());
    println!("{}", k.get_min());
    k.push(-2147483648);
    println!("{}", k.top());
    println!("{}", k.get_min());
    k.pop();
    println!("{}", k.get_min());
}
