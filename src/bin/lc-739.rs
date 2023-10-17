fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    // neetcode's simpler solution!
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut cstack = Vec::new();

        let mut it = temperatures.into_iter().enumerate();
        cstack.push(it.next().unwrap());

        for c in it {
            let i = c.0;
            let t = c.1;
            while !cstack.is_empty() && t > cstack.last().unwrap().1 {
                let _c = cstack.pop().unwrap();
                let _i = _c.0;
                result[_i] = (i - _i) as i32;
            }
            cstack.push(c);
        }
        result
    }
    // pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    //     let mut result = Vec::new();
    //     let mut cstack = Vec::new();

    //     let mut ti = temperatures.into_iter().rev();
    //     result.push(0);
    //     cstack.push((ti.next().unwrap(), 0));

    //     'next_t: for t in ti {
    //         let mut r = 0;
    //         while let Some(tc) = cstack.last() {
    //             let _t = tc.0;
    //             let c = tc.1;
    //             if t < _t {
    //                 r += 1;
    //                 result.push(r);
    //                 cstack.push((t, r));
    //                 continue 'next_t;
    //             } else {
    //                 cstack.pop();
    //                 r += c;
    //             }
    //         }
    //         cstack.push((t, 0));
    //         result.push(0);
    //     }
    //     result.into_iter().rev().collect()
    // }
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![30, 40, 50, 60]),
        vec![1, 1, 1, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![30, 60, 90]),
        vec![1, 1, 0]
    )
}
