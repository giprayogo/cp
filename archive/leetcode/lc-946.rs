impl Solution {
    fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut push_iter = pushed.into_iter().peekable();
        let mut pop_iter = popped.into_iter().peekable();
        while let Some(push) = push_iter.peek() {
            if let Some(tail) = stack.last() {
                if let Some(peek) = pop_iter.peek() {
                    if tail == peek {
                        stack.pop();
                        pop_iter.next();
                    } else {
                        stack.push(*push);
                        push_iter.next();
                    }
                }
            } else {
                stack.push(*push);
                push_iter.next();
            }
        }
        while let Some(e) = stack.pop() {
            if let Some(next) = pop_iter.next() {
                if e != next {
                    return false;
                }
            }
        }
        true
    }
}

// Lesson: can't simply simulate stack behaviour with a set of pointers! (early optimization again...)

struct Solution;

fn main() {
    assert!(Solution::validate_stack_sequences(
        vec![2, 1, 0],
        vec![1, 2, 0]
    ));
    assert!(Solution::validate_stack_sequences(
        vec![1, 2, 3, 4, 5],
        vec![4, 5, 3, 2, 1]
    ));
    assert!(Solution::validate_stack_sequences(
        vec![0, 2, 1],
        vec![0, 1, 2]
    ));
    assert!(Solution::validate_stack_sequences(vec![1, 2], vec![2, 1]));
    assert!(!Solution::validate_stack_sequences(vec![1], vec![2]));
    assert!(Solution::validate_stack_sequences(vec![1], vec![1]));
    assert!(!Solution::validate_stack_sequences(
        vec![1, 2, 3, 4, 5],
        vec![4, 5, 3, 1, 2]
    ));
}
