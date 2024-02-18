struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // The missing link: Kadane algorithm: probably I need to read more...
        let mut max = i32::MIN;
        let mut cur_product = 1;
        let mut cur_min_product = 1;

        for num in nums {
            let temp = cur_product;
            cur_product = (num * cur_product).max(num).max(num * cur_min_product);
            cur_min_product = (num * cur_min_product).min(num).min(num * temp);
            max = max.max(cur_product).max(cur_min_product); // NOTE: Last max unnecessary!
        }

        // more symmetric solution
        // let mut products: Vec<i32> = vec![1; nums.len()];
        // let mut max = i32::MIN;
        // let mut products: Vec<i32> = nums.clone();
        // let mut max = *products.iter().max().unwrap();

        // for d in 1..=(nums.len()) {
        //     for i in 0..=(nums.len() - d) {
        //         // Finally accepted but sloooow; does highlight how... wrong;
        //         // the previous complicated solutions were; wrong attempt at DP. Think.
        //         products[i] *= nums[i + d - 1];
        //         max = max.max(products[i]);
        //         // NOTE: Sometimes the maximum is not its immediate "child",
        //         // so this does not work...
        //         // let v = (products[i] * nums[i + d - 1])
        //         //     .max(products[i])
        //         //     .max(products[i + 1]);
        //         // NOTE: This product will take a looooooooong time for loooooong array
        //         // let v = nums[i..(i + d)]
        //         //     .iter()
        //         //     .product::<i32>()
        //         //     .max(products[i])
        //         //     .max(products[i + 1]);
        //         // products[i] = v;
        //     }
        // }
        // products[0]
        max
    }
}

fn main() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![-2]), -2);
    assert_eq!(Solution::max_product(vec![-4, -3]), 12);
    assert_eq!(Solution::max_product(vec![0, 2]), 2);
    assert_eq!(Solution::max_product(vec![-4, -3, -2]), 12)
}
