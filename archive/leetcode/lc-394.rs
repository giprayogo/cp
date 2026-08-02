use std::iter::Peekable;

impl Solution {
    pub fn digit<'a, I>(ci: &mut Peekable<I>) -> Vec<char>
    where
        I: Iterator<Item = &'a char>,
    {
        let mut chars = Vec::new();
        while let Some(c) = ci.peek() {
            match c {
                '0'..='9' => {
                    if let Some(c) = ci.next() {
                        chars.push(*c);
                    }
                }
                '[' => {
                    let n: usize = chars.iter().collect::<String>().parse().unwrap();
                    chars.clear();
                    ci.next();
                    if let Some(c) = ci.peek() {
                        match c {
                            '0'..='9' => chars.extend(Solution::digit(ci).repeat(n)),
                            'a'..='z' => chars.extend(Solution::alphabet(ci).repeat(n)),
                            _ => panic!("Unexpected character: {c}"),
                        }
                    }
                    return chars;
                }
                _ => panic!("Unexpected character: {c}"),
            }
        }
        chars
    }

    pub fn alphabet<'a, I>(ci: &mut Peekable<I>) -> Vec<char>
    where
        I: Iterator<Item = &'a char>,
    {
        let mut chars = Vec::new();
        while let Some(c) = ci.peek() {
            match c {
                'a'..='z' => {
                    if let Some(c) = ci.next() {
                        chars.push(*c);
                    }
                }
                '0'..='9' => {
                    chars.extend(Solution::digit(ci));
                }
                ']' => {
                    ci.next();
                    // consume all closing brackets? how about recursive brackets?
                    return chars;
                }
                _ => panic!("Unexpected character: {c}"),
            }
        }
        chars
    }

    pub fn decode_string(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut ci = chars.iter().peekable();

        let mut decoded_chars = Vec::new();
        while let Some(c) = ci.peek() {
            match c {
                '0'..='9' => {
                    decoded_chars.extend(Solution::digit(&mut ci));
                }
                'a'..='z' => {
                    decoded_chars.extend(Solution::alphabet(&mut ci));
                }
                _ => panic!("Unexpected character: {c}"),
            }
        }
        decoded_chars.iter().collect()
    }
}

// A recursive descent parser

struct Solution;

fn main() {
    Solution::decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".to_string());
    assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc");
    assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc");
    assert_eq!(
        Solution::decode_string("2[abc]3[cd]ef".to_string()),
        "abcabccdcdcdef"
    );
}
