use std::collections::HashMap;

struct Solution;

impl Solution {
    // Actually not TLE?? But let's see if I missed some tricks... -> actually not!
    // If any probably I overthinked about grouping words by their length...
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let chars: Vec<_> = s.chars().collect();

        let mut char_dict: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
        for word in word_dict {
            let _chars: Vec<char> = word.chars().collect();
            char_dict
                .entry(_chars.len())
                .and_modify(|x| x.push(word.chars().collect()))
                .or_insert(vec![word.chars().collect()]);
        }
        let n_ok = chars.len() + 1;
        let mut ok = vec![false; n_ok];
        ok[n_ok - 1] = true;
        for i in (0..n_ok).rev() {
            for j in char_dict.keys() {
                if i + j <= chars.len() && ok[i + j] {
                    for word in char_dict[j].iter() {
                        if *word == chars[i..i + j] {
                            ok[i] = true;
                            break;
                        }
                    }
                }
            }
        }

        ok[0]
    }
}

fn main() {
    assert!(Solution::word_break(
        "leetcode".into(),
        vec!["leet".into(), "code".into()]
    ));
    assert!(Solution::word_break(
        "applepenapple".into(),
        vec!["apple".into(), "pen".into()]
    ));
    assert!(!Solution::word_break(
        "catsandog".into(),
        vec![
            "cats".into(),
            "dog".into(),
            "sand".into(),
            "and".into(),
            "cat".into()
        ]
    ));
}
