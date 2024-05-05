/// Practice coding for common algorithms

/// Knuth-Morris-Pratt algorithm for string searching.
/// Based on https://cp-algorithms.com/string/prefix-function.html.
///
/// Naive function
fn prefix_function_naive(s: &str) -> Vec<i32> {
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    let mut pi = vec![0; n];
    for i in 0..n {
        let sub = &c[0..=i];
        'k: for k in 1..(i + 1) {
            for (f, b) in sub.iter().take(k).zip(sub.iter().skip(i + 1 - k)) {
                if f != b {
                    continue 'k;
                }
            }
            pi[i] = k as i32;
        }
    }
    pi
}

fn main() {
    // assert eq for all the implementations
    assert_eq!(prefix_function_naive("abcabcd"), [0, 0, 0, 1, 2, 3, 0]);
    assert_eq!(prefix_function_naive("aabaaab"), [0, 1, 0, 1, 2, 2, 3]);
}
