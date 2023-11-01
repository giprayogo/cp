use std::collections::HashSet;

fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = s.chars();
        let mut r = s.chars();
        // let mut charset = HashSet::new();
        let mut maxlen = 0;
        let mut len = 0;

        for c in r {
            // looks tempting to be turned into while, but it is slower!
            if charset.contains(&c) {
                loop {
                    let lc = l.next().unwrap();
                    charset.remove(&lc);
                    len -= 1;
                    if lc == c {
                        break;
                    }
                }
            }
            charset.insert(c);
            len += 1;

            maxlen = maxlen.max(len)
        }
        maxlen
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("a".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("au".into()), 2);
    assert_eq!(Solution::length_of_longest_substring("tmmzuxt".into()), 5);
}