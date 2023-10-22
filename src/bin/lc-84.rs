fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    // first accepted solution
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut max_area = 0;

        let mut notdrop = vec![];
        heights.push(0);
        let mut hi = heights.into_iter().enumerate();
        notdrop.push(hi.next().unwrap());

        // todo: more elegant
        for (i, h) in hi {
            if notdrop.is_empty() || h > notdrop.last().unwrap().1 {
                notdrop.push((i, h))
            } else {
                // todo: more elegant
                let mut lastdrop = i;
                while !notdrop.is_empty() && notdrop.last().unwrap().1 > h {
                    let e = notdrop.pop().unwrap();
                    // a bit more memory efficient? not worth it
                    // max_area = (e.1 * (i - e.0) as i32).max(max_area);
                    // faster
                    let area = e.1 * (i - e.0) as i32;
                    if area > max_area {
                        max_area = area;
                    }
                    lastdrop = e.0;
                }
                // notdrop.push((i, h)) // 1 start from 0 not 1!
                notdrop.push((lastdrop, h));
                // match notdrop.last() {
                //     Some(v) => notdrop.push((v.0, h)),
                //     None => notdrop.push((i, h))
                // }
            }
        }
        max_area
    }

    // prototype quick solutoin first (even if TLE)
    // and TLE indeed!
    // pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
    //     heights.push(0);
    //     let mut h_count = vec![0; 10001];
    //     let mut max_area = 0;
    //     let mut last_height = 0;
    //     for h in heights {
    //         if h < last_height {
    //             for i in h..last_height {
    //                 let area = h_count[i as usize] * (i + 1);
    //                 if area > max_area {
    //                     max_area = area;
    //                 }
    //                 h_count[i as usize] = 0;
    //             }
    //         }
    //         for i in 0..h {
    //             h_count[i as usize] += 1;
    //         }
    //         last_height = h;
    //     }

    //     max_area
    // }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
}
