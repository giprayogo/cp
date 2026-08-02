use std::collections::HashMap;

struct Solution;

impl Solution {
    // Accept! Now optimizations...
    #[allow(clippy::too_many_arguments)]
    pub fn is_interleave_dfs(s1: String, s2: String, s3: String) -> bool {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let c3: Vec<char> = s3.chars().collect();

        if c1.len() + c2.len() != c3.len() {
            return false;
        }

        let mut memo = HashMap::new();
        fn dfs(
            memo: &mut HashMap<(isize, isize, isize), bool>,
            c1: &[char],
            c2: &[char],
            c3: &[char],
            i1: isize,
            i2: isize,
            i3: isize,
            left: bool,
        ) -> bool {
            match memo.get(&(i1, i2, i3)) {
                Some(v) => *v,
                None => {
                    if i3 as usize >= c3.len() {
                        memo.insert((i1, i2, i3), true);
                        return true;
                    }
                    if (i1 >= 0 && i1 as usize == c1.len()) || (i2 >= 0 && i2 as usize == c2.len())
                    {
                        memo.insert((i1, i2, i3), false);
                        return false;
                    }

                    let char1 = c1.get(i1 as usize);
                    let char2 = c2.get(i2 as usize);
                    let char3 = c3.get(i3 as usize);

                    if (left && char1 != char3) || (!left && char2 != char3) {
                        memo.insert((i1, i2, i3), false);
                        return false;
                    }

                    let result = dfs(memo, c1, c2, c3, i1 + 1, i2, i3 + 1, true)
                        || dfs(memo, c1, c2, c3, i1, i2 + 1, i3 + 1, false);
                    memo.insert((i1, i2, i3), result);
                    result
                }
            }
        }
        dfs(&mut memo, &c1, &c2, &c3, 0, -1, 0, true)
            || dfs(&mut memo, &c1, &c2, &c3, -1, 0, 0, false)
    }

    // Well different implementation of DP, really!
    fn is_interleave_dp(s1: String, s2: String, s3: String) -> bool {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let c3: Vec<char> = s3.chars().collect();

        if c1.len() + c2.len() != c3.len() {
            return false;
        }

        let ni = c2.len() + 1;
        let nj = c1.len() + 1;
        let mut dp = vec![false; nj];
        dp[0] = true;
        for i in 0..ni {
            let mut _dp = vec![false; nj];
            for j in 0..nj {
                let _i = i.checked_sub(1);
                let _j = j.checked_sub(1);
                if let Some(__i) = _i {
                    _dp[j] = _dp[j] || (dp[j] && c3[__i + j] == c2[__i]);
                } else {
                    _dp[j] = _dp[j] || dp[j];
                }
                if let Some(__j) = _j {
                    _dp[j] = _dp[j] || (_dp[__j] && c3[i + __j] == c1[__j]);
                }
            }
            dp = _dp;
        }
        *dp.last().unwrap()
    }
}

fn main() {
    for f in [Solution::is_interleave_dfs, Solution::is_interleave_dp] {
        // for f in [Solution::is_interleave_dfs] {
        assert!(f(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ));
        assert!(!f(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ));
        assert!(f("".to_string(), "".to_string(), "".to_string()));
        assert!(!f("a".to_string(), "".to_string(), "c".to_string()));
        assert!(!f("a".to_string(), "b".to_string(), "a".to_string()));
        assert!(!f("ab".to_string(), "bc".to_string(), "bbac".to_string()));
    }
}
