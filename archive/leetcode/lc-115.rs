use std::collections::HashMap;

struct Solution;

impl Solution {
    // As expected, correct but not the best!
    pub fn num_distinct_1(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut memo = HashMap::new();
        fn dfs(
            memo: &mut HashMap<(usize, usize), i32>,
            s: &Vec<char>,
            t: &Vec<char>,
            i: Option<usize>,
            j: Option<usize>,
        ) -> i32 {
            let j = match j {
                Some(j) => j,
                None => return 1,
            };
            let i = match i {
                Some(i) => i,
                None => return 0,
            };
            match memo.get(&(i, j)) {
                Some(v) => *v,
                None => {
                    let _i = (i + 1 < s.len()).then_some(i + 1);
                    let mut res = 0;
                    if s[i] == t[j] {
                        let _j = (j + 1 < t.len()).then_some(j + 1);
                        res += dfs(memo, s, t, _i, _j);
                    }
                    res += dfs(memo, s, t, _i, Some(j));
                    memo.insert((i, j), res);
                    res
                }
            }
        }
        dfs(&mut memo, &s, &t, Some(0), Some(0))
    }

    // DP with exact transfer from above
    #[allow(clippy::needless_range_loop)]
    pub fn num_distinct_2(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut num = vec![1; s.len()];
        for j in 0..t.len() {
            let mut _num = vec![0; s.len()];
            for i in 0..s.len() {
                let _i = i.checked_sub(1);
                if s[i] == t[j] {
                    _num[i] += match _i {
                        Some(_i) => num[_i],
                        None => match j {
                            0 => 1,
                            _ => 0,
                        },
                    };
                }
                _num[i] += match _i {
                    Some(_i) => _num[_i],
                    None => 0,
                };
            }
            num = _num;
        }
        *num.last().unwrap()
    }

    // Optimized
    #[allow(clippy::needless_range_loop)]
    pub fn num_distinct_3(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut num = vec![1; s.len()];
        let mut _num = vec![0; s.len()];
        if s[0] == t[0] {
            _num[0] = 1;
        }
        for j in 0..t.len() {
            for i in 1..s.len() {
                let _i = i - 1;
                if s[i] == t[j] {
                    _num[i] += num[_i];
                }
                _num[i] += _num[_i];
            }
            num = _num;
            _num = vec![0; s.len()];
        }
        *num.last().unwrap()
    }
}

fn main() {
    for f in [
        Solution::num_distinct_1,
        Solution::num_distinct_2,
        Solution::num_distinct_3,
    ] {
        assert_eq!(f("rabbbit".to_string(), "rabbit".to_string()), 3);
        assert_eq!(f("babgbag".to_string(), "bag".to_string()), 5);
        assert_eq!(f("ddd".to_string(), "dd".to_string()), 3);
        assert_eq!(f(
            "adbdadeecadeadeccaeaabdabdbcdabddddabcaaadbabaaedeeddeaeebcdeabcaaaeeaeeabcddcebddebeebedaecccbdcbcedbdaeaedcdebeecdaaedaacadbdccabddaddacdddc".to_string(),
            "bcddceeeebecbc".to_string()), 700531452)
    }
}
