impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let c1: Vec<_> = num1
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .rev()
            .collect();
        let c2: Vec<_> = num2
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .rev()
            .collect();
        let mut res = vec![0; c1.len() + c2.len()];
        for (shift, x) in c2.iter().enumerate() {
            let mut carry = 0;
            let mut _res = vec![0; c1.len() + 1];
            for (i, y) in c1.iter().enumerate() {
                let xy = x * y + carry;
                _res[i] = xy % 10;
                carry = xy / 10;
            }
            if carry != 0 {
                *_res.last_mut().unwrap() = carry;
                carry = 0;
            }
            let _n = _res.len();
            for (a, b) in _res.into_iter().zip(res.iter_mut().skip(shift)) {
                let c = a + *b + carry;
                *b = c % 10;
                carry = c / 10;
            }
            if carry != 0 {
                res[shift + _n] = carry;
            }
        }
        let s: Vec<_> = res
            .into_iter()
            .map(|x| char::from_digit(x, 10).unwrap())
            .rev()
            .collect();
        for i in 0..s.len() {
            if s[i] != '0' {
                return s[i..].iter().collect();
            }
        }
        "0".into()
    }

    // neetcode: probably my math get rusty or I never learned that way... but it is not necessary to carry the carries within the mul logic!
    pub fn multiply_2(num1: String, num2: String) -> String {
        let c1: Vec<_> = num1
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .rev()
            .collect();
        let c2: Vec<_> = num2
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .rev()
            .collect();
        let mut res = vec![0; c1.len() + c2.len()];

        for (shift, x) in c2.iter().enumerate() {
            for (i, y) in c1.iter().enumerate() {
                let xy = x * y;
                res[shift + i] += xy;
                res[shift + i + 1] += res[shift + i] / 10; // spillover carry
                res[shift + i] %= 10;
            }
        }

        let s: Vec<_> = res
            .into_iter()
            .map(|x| char::from_digit(x, 10).unwrap())
            .rev()
            .collect();
        for i in 0..s.len() {
            if s[i] != '0' {
                return s[i..].iter().collect();
            }
        }
        "0".into()
    }
}

struct Solution;

fn main() {
    for f in [Solution::multiply, Solution::multiply_2] {
        assert_eq!(f("2".into(), "3".into()), "6");
        assert_eq!(f("123".into(), "456".into()), "56088");
    }
}
