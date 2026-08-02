#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut fi = s.chars();
        let mut bi = s.chars().rev();
        'all: while let (Some(cf), Some(bf)) = (fi.next(), bi.next()) {
            let mut cf = cf.to_ascii_lowercase();
            let mut bf = bf.to_ascii_lowercase();
            while !cf.is_alphanumeric() {
                cf = match fi.next() {
                    Some(c) => c.to_ascii_lowercase(),
                    None => break 'all,
                };
            }
            while !bf.is_alphanumeric() {
                bf = match bi.next() {
                    Some(c) => c.to_ascii_lowercase(),
                    None => break 'all,
                };
            }
            if cf != bf {
                return false;
            }
        }
        true
    }
}

fn main() {}

#[test]
fn test_solution() {
    let s = "A man, a plan, a canal: Panama";
    assert!(Solution::is_palindrome(s.into()));
    let s = "race a car";
    assert!(!Solution::is_palindrome(s.into()));
    let s = " ";
    assert!(Solution::is_palindrome(s.into()));
    let s = ".,";
    assert!(Solution::is_palindrome(s.into()));
}