struct Solution;

impl Solution {
    // Correct but meh
    pub fn can_complete_circuit_1(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.len() == 1 {
            return match gas.first().unwrap() - cost.first().unwrap() >= 0 {
                true => 0,
                false => -1,
            };
        }
        let mut _delta = gas.last().unwrap() - cost.last().unwrap();
        for (i, (g, c)) in gas.iter().zip(cost.iter()).enumerate() {
            let delta = g - c;
            if delta.is_positive() && _delta <= 0 {
                let mut acc = 0;
                let can = 'can: {
                    for j in 0..gas.len() {
                        let _i = (i + j) % gas.len();
                        acc += gas[_i] - cost[_i];
                        if acc < 0 {
                            break 'can false;
                        }
                    }
                    true
                };
                if can {
                    return i as i32;
                }
            }
            _delta = delta;
        }
        -1
    }

    // I think I don't need to do the internal loop; better but still like I'm missing something!
    pub fn can_complete_circuit_2(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.len() == 1 {
            return match gas.first().unwrap() - cost.first().unwrap() >= 0 {
                true => 0,
                false => -1,
            };
        }
        let mut _delta = gas.last().unwrap() - cost.last().unwrap();
        let mut l = 0;
        while l < gas.len() {
            let mut delta = gas[l] - cost[l];
            if delta.is_positive() && _delta <= 0 {
                let mut acc = 0;
                let can = 'can: {
                    for d in 0..gas.len() {
                        let r = (l + d) % gas.len();
                        acc += gas[r] - cost[r];
                        if acc < 0 {
                            l = (l + 1).max(r + 1);
                            delta = gas[r] - cost[r];
                            break 'can false;
                        }
                    }
                    true
                };
                if can {
                    return l as i32;
                }
            } else {
                l += 1;
            }
            _delta = delta;
        }
        -1
    }

    // neetcode: answer is unique; so I should just move one-by-one such that
    // the (cost) sum is exactly positive (>=0); moving the end forward otherwise.
    // If the end and start met then I have found the starting point!
    // Learn also how to make sure I don't miss thing by one index
    // Probably faster? Unfortunately not thorough for all test cases!
    // Lesson: Don't overthink it. Other people have same solution as mine...early optimization habit.
    pub fn can_complete_circuit_3(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        println!("==");
        let mut start = gas.len() - 1;
        let mut end = 0;
        let mut sum = gas[start] - cost[start];
        while end <= start {
            while sum < 0 && end <= start {
                if let Some(_start) = start.checked_sub(1) {
                    start = _start;
                } else {
                    return -1;
                }
                sum += gas[start] - cost[start];
            }

            if start == end {
                return start as i32;
            }
            sum += gas[end] - cost[end];
            end += 1;
        }
        -1
    }
}

fn main() {
    for f in [
        Solution::can_complete_circuit_1,
        Solution::can_complete_circuit_2,
        Solution::can_complete_circuit_3,
    ] {
        assert_eq!(f(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
        assert_eq!(f(vec![2, 3, 4], vec![3, 4, 3]), -1);
        assert_eq!(f(vec![1], vec![100]), -1);
        assert_eq!(f(vec![100], vec![1]), 0);
        assert_eq!(f(vec![2], vec![2]), 0);
        assert_eq!(f(vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1]), 4);
        assert_eq!(f(vec![2, 0, 0, 0, 0], vec![0, 1, 0, 0, 0]), 0);
    }
}
