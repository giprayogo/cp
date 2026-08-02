use std::collections::HashMap;

impl Solution {
    fn sort_array(nums: Vec<i32>) -> i32 {
        // distance for 0 in front
        let mut index: HashMap<_, _> = nums
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i as i32))
            .collect();
        let mut index_2 = index.clone();
        let mut cur = nums.iter().position(|&x| x == 0).unwrap() as i32;
        let mut cur_2 = cur;
        let mut distance_1 = 0;
        'a: loop {
            // parked; seek first out of place element
            if cur == 0 {
                for _i in 1..nums.len() as i32 {
                    let next = index[&_i];
                    if next != _i {
                        index.entry(0).and_modify(|e| *e = next);
                        index.entry(_i).and_modify(|e| *e = 0);
                        cur = next;
                        distance_1 += 1;
                        continue 'a;
                    }
                }
                break;
            }
            let next = index[&cur];
            index.entry(0).and_modify(|e| *e = next);
            index.entry(cur).and_modify(|e| *e = cur);
            cur = next;
            distance_1 += 1;
        }

        // distance for zero in the back
        let mut distance_2 = 0;
        'a: loop {
            // parked; seek first out of place element
            if cur_2 == nums.len() as i32 - 1 {
                for _i in 1..nums.len() as i32 {
                    let next = index_2[&_i];
                    if next != _i - 1 {
                        index_2.entry(0).and_modify(|e| *e = next);
                        index_2.entry(_i).and_modify(|e| *e = nums.len() as i32 - 1);
                        cur_2 = next;
                        distance_2 += 1;
                        continue 'a;
                    }
                }
                break;
            }
            let next = index_2[&(cur_2 + 1)];
            index_2.entry(0).and_modify(|e| *e = next);
            index_2.entry(cur_2 + 1).and_modify(|e| *e = cur_2);
            cur_2 = next;
            distance_2 += 1;
        }
        distance_1.min(distance_2)
    }
}

// NOTE: Overthink. Indeed I need to compare to sorted solution. Stop overthinking!
// 2nd NOTE: EACH ELEMENT from 0..n -> they are they own index. No need for sort
// 3rd overthink: unnecessary optimization? what's wrong with physically swapping the element
// if I end up having to have visited element anyway (that O(N^2) tho...)?
// Or: why don't I evolve the index????? Anyway.

struct Solution;

fn main() {
    assert_eq!(Solution::sort_array(vec![4, 3, 2, 1, 0]), 4);
    assert_eq!(Solution::sort_array(vec![4, 2, 0, 3, 1]), 3);
    assert_eq!(Solution::sort_array(vec![1, 0, 2, 4, 3]), 2);
    assert_eq!(Solution::sort_array(vec![0, 1, 2, 3, 4]), 0);
    assert_eq!(Solution::sort_array(vec![0, 1]), 0);
    assert_eq!(Solution::sort_array(vec![1, 0]), 0);
}
