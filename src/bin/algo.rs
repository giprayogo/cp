/// Practice coding for common algorithms

/// Knuth-Morris-Pratt algorithm for string searching.
/// Based on https://cp-algorithms.com/string/prefix-function.html.
///
/// Naive function
#[allow(clippy::needless_range_loop)]
fn prefix_function_naive(s: &str) -> Vec<i32> {
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    let mut pi = vec![0; n];
    for i in 0..n {
        'k: for k in 1..=i {
            let prefix = c.iter().take(k);
            let suffix = c.iter().skip(i - k + 1).take(k);
            for (a, b) in prefix.zip(suffix) {
                if a != b {
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
