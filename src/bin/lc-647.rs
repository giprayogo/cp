struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<_> = s.chars().collect();
        let mut count = 0;
        for i in 0..chars.len() {
            let even = (0..=i).rev().zip(i..chars.len());
            let odd = (0..=i).rev().zip(i + 1..chars.len());
            for (start, end) in even {
                if chars[start] != chars[end] {
                    break;
                }
                count += 1;
            }
            for (start, end) in odd {
                if chars[start] != chars[end] {
                    break;
                }
                count += 1;
            }
        }
        count
    }
}

fn main() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    assert_eq!(Solution::count_substrings("hello".to_string()), 6);
}
