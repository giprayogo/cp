// use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    // got hint from neetcode how to do it properly. Technically I got the idea
    // correct (how to represent the scan for palindrome), but was obsessed by cache :)
    // Also I missed the fact that; if expanding symmetrically, if the center is not
    // palindrome then the expanded string is also not!
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<_> = s.chars().collect();
        let mut max_len = 0;
        let mut longest = (0, 0);
        for i in 0..chars.len() {
            let a = (0..=i).rev().zip(i..chars.len());
            for (s, e) in a {
                if chars[s] != chars[e] {
                    break;
                }
                let length = e - s;
                if length > max_len {
                    max_len = length;
                    longest = (s, e);
                }
                max_len = max_len.max(e - s);
            }
            let a = (0..=i).rev().zip(i + 1..chars.len());
            for (s, e) in a {
                if chars[s] != chars[e] {
                    break;
                }
                let length = e - s;
                if length > max_len {
                    max_len = length;
                    longest = (s, e);
                }
                max_len = max_len.max(e - s);
            }
        }
        chars[longest.0..=longest.1].iter().collect()
    }

    // NOTE: About these solutions: apparently I have obsession of caching everything
    // when... for these cases it is definitely possible to avoid accessing the same
    // element twice!
    /*     pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<_> = s.chars().collect();
        let mut cache = HashMap::new();

        let mut stack = VecDeque::new();
        stack.push_back((0, chars.len()));
        while let Some((start, end)) = stack.pop_front() {
            if Solution::is_palindrome(&mut cache, &chars, start, end) {
                return chars[start..end].iter().collect();
            }
            if end - start > 1 {
                stack.push_back((start, end - 1));
                stack.push_back((start + 1, end));
            }
        }
        unreachable!()
    }

    fn is_palindrome(
        cache: &mut HashMap<(usize, usize), bool>,
        chars: &[char],
        start: usize,
        end: usize,
    ) -> bool {
        match cache.get(&(start, end)) {
            Some(v) => *v,
            None => {
                let mut i = start;
                let mut j = end;
                while j > i {
                    if chars[i] != chars[j - 1] {
                        for (_i, _j) in (start..i).zip(j..end) {
                            cache.insert((_i, _j), false);
                        }
                        // cache.insert((i, j), false);
                        return false;
                    }
                    i += 1;
                    j -= 1;
                }
                true
                // Problem with this implementation: if palindrome, will run through the end regardless!
                // let forward = chars[start..end].iter(); let backward = chars[start..end].iter().rev();
                // for (f, b) in forward.zip(backward) {
                //     if f != b {
                //         // for (_f, _b) in chars[start..=f].zip(chars[b..=stop]) {
                //         //     cache.insert((_f, _b), false);
                //         // }
                //         cache.insert((start, end), false);
                //         return false;
                //     }
                // }
                // cache.insert((start, end), true);
                // true
            }
        }
    } */
}

fn main() {
    assert_eq!(Solution::longest_palindrome("babad".into()), "bab");
    assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
    println!(
        "{}",
        Solution::longest_palindrome("cbbdaslkdfjaaabbbbbbbbaasldkfjs".into())
    );
    println!("{}", Solution::longest_palindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".into()));
}
