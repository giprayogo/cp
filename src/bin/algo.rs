/// Practice coding for common algorithms

/// Knuth-Morris-Pratt algorithm for string searching.
/// Based on https://cp-algorithms.com/string/prefix-function.html.
///
/// Naive function
#[allow(clippy::needless_range_loop)]
fn prefix_function_naive(s: &str) -> Vec<usize> {
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    let mut pi = vec![0; n];
    for i in 1..n {
        for k in 1..=i {
            let prefix = c.iter().take(k);
            let suffix = c.iter().skip(i - k + 1).take(k);
            if prefix.eq(suffix) {
                pi[i] = k;
            }
        }
    }
    pi
}

fn prefix_function_optimized(s: &str) -> Vec<usize> {
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && c[i] != c[j] {
            // Note that here it is not i - 1 but j - 1!
            // i.e. I'm finding another prefix/suffix within current best prefix/suffix,
            // where the suffix property also hold. I was being dense! Paper works!
            j = pi[j - 1];
        }
        if c[i] == c[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

/// Search for pattern p in string s
#[allow(clippy::needless_range_loop)]
fn kmp_search(p: &str, s: &str) -> Vec<usize> {
    let c: Vec<char> = s.chars().collect();
    let d: Vec<char> = p.chars().collect();
    let table = prefix_function_optimized(p);
    let n = c.len();
    let m = table.len();

    let mut results = Vec::with_capacity(n);
    let mut j = 0; // -length-
    for i in 0..n {
        while j > 0 && c[i] != d[j] {
            j = table[j - 1];
        }
        if c[i] == d[j] {
            j += 1;
        }
        if j == m {
            results.push(i + 1 - m);
            j = table[j - 1];
        }
    }
    results
}

fn main() {
    for f in [prefix_function_naive, prefix_function_optimized] {
        assert_eq!(f("a"), [0]);
        assert_eq!(f("aa"), [0, 1]);
        assert_eq!(f("aaa"), [0, 1, 2]);
        assert_eq!(f("aaaa"), [0, 1, 2, 3]);
        assert_eq!(f("abcabcd"), [0, 0, 0, 1, 2, 3, 0]);
        assert_eq!(f("aabaaab"), [0, 1, 0, 1, 2, 2, 3]);
    }

    assert_eq!(kmp_search("abc", "abcabc"), [0, 3]);
    assert_eq!(kmp_search("a", "abcabc"), [0, 3]);
    assert_eq!(kmp_search("aa", "aaa"), [0, 1]);
}
