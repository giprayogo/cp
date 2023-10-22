fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.len() == 1 {
            return 1;
        }
        let mut ps = position
            .into_iter()
            .zip(speed.into_iter())
            .collect::<Vec<_>>();
        ps.sort_by(|&a, &b| b.0.partial_cmp(&a.0).unwrap());
        let mut fleet_head = Vec::new();
        for (left_p, left_s) in ps {
            if fleet_head.is_empty() {
                fleet_head.push((left_p, left_s));
                continue;
            }

            let head_p = fleet_head.last().unwrap().0;
            let head_s = fleet_head.last().unwrap().1;
            let delta_s = left_s - head_s;
            if delta_s <= 0 {
                fleet_head.push((left_p, left_s));
                continue;
            }
            let delta_p = head_p - left_p;
            // let catch_in = delta_p / delta_s + (delta_p % delta_s != 0) as i32;
            let catch_in = delta_p as f64 / delta_s as f64;
            if head_p as f64 + catch_in * head_s as f64 > target as f64 {
                fleet_head.push((left_p, left_s));
            }
        }
        fleet_head.len() as i32
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
        3
    );
    assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
    assert_eq!(Solution::car_fleet(10, vec![4, 6], vec![3, 2]), 1);
    assert_eq!(Solution::car_fleet(16, vec![11, 14, 13, 6], vec![2, 2, 6, 7]), 2);
    assert_eq!(Solution::car_fleet(21, vec![1,15,6,8,18,14,16,2,19,17,3,20,5], vec![8,5,5,7,10,10,7,9,3,4,4,10,2]), 7)
}
