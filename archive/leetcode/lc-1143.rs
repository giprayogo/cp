struct Solution;

impl Solution {
    // Accepted! But can probably be faster still...
    #[allow(clippy::needless_range_loop)]
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();
        let ni = text1.len();
        let nj = text2.len();
        let mut array = vec![vec![0; nj]; ni];
        for i in 0..ni {
            for j in 0..nj {
                // neetcode: no need to max: if character match it -surely- adds to the sequence!
                // let mut value = (text1[i] == text2[j]) as i32;
                // let left = if j != 0 { array[i][j - 1] } else { 0 };
                // let up = if i != 0 { array[i - 1][j] } else { 0 };
                // let diag = if i != 0 && j != 0 {
                //     array[i - 1][j - 1]
                // } else {
                //     0
                // };
                // value = left.max(up).max(value + diag);
                // array[i][j] = value;
                if text1[i] == text2[j] {
                    array[i][j] = if i != 0 && j != 0 {
                        array[i - 1][j - 1] + 1
                    } else {
                        1
                    };
                } else {
                    let left = if j != 0 { array[i][j - 1] } else { 0 };
                    let up = if i != 0 { array[i - 1][j] } else { 0 };
                    array[i][j] = left.max(up);
                }
            }
        }
        *array.last().unwrap().last().unwrap()
    }
}

fn main() {
    assert_eq!(
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
        3
    );
    assert_eq!(
        Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
        3
    );
    assert_eq!(
        Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
        0
    );
    assert_eq!(
        Solution::longest_common_subsequence("bsbininm".to_string(), "jmjkbkjkv".to_string()),
        1
    );
    assert_eq!(
        Solution::longest_common_subsequence("abb".to_string(), "b".to_string()),
        1
    );
}
