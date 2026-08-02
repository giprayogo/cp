struct Solution;

impl Solution {
    // Lessons from debugging: offset by 1 can be really annoying.
    // take a deep look whenever this could have happened
    #[allow(clippy::needless_range_loop)]
    pub fn min_distance_1(word1: String, word2: String) -> i32 {
        // println!("-- {} {}", word1, word2);
        let mut chars1: Vec<char> = word1.chars().collect();
        let mut chars2: Vec<char> = word2.chars().collect();
        let mut ni = chars1.len() + 1;
        let mut nj = chars2.len() + 1;
        if nj > ni {
            std::mem::swap(&mut chars1, &mut chars2);
            std::mem::swap(&mut ni, &mut nj);
        }
        let mut matrix = vec![vec![0; nj]; ni];
        // let debug_matrix = vec![vec![1; nj]; ni];
        for i in 0..ni {
            for j in 0..nj {
                // if chars1[i] == chars2[j] {
                //     debug_matrix[i][j] = 0;
                // }
                // increase this part's efficiency...
                let _i = i.checked_sub(1);
                let _j = j.checked_sub(1);
                let del = match _i {
                    Some(_i) => matrix[_i][j] + 1,
                    None => i32::MAX,
                };
                let ins = match _j {
                    Some(_j) => matrix[i][_j] + 1,
                    None => i32::MAX,
                };
                let rep = match (_i, _j) {
                    (Some(_i), Some(_j)) => {
                        if chars1[_i] == chars2[_j] {
                            matrix[i - 1][j - 1]
                        } else {
                            matrix[i - 1][j - 1] + 1
                        }
                    }
                    _ => i32::MAX,
                };
                let edit = match del.min(ins).min(rep) {
                    // i32::MAX => (chars1[i] != chars2[j]) as i32,
                    i32::MAX => 0,
                    v => v,
                };
                matrix[i][j] = edit;
            }
        }
        // println!("<<<<<<<<<<<<<");
        // for row in debug_matrix.iter() {
        //     for e in row.iter() {
        //         print!("{e:2} ");
        //     }
        //     println!();
        // }
        // println!("-------------");
        // for row in matrix.iter() {
        //     for e in row.iter() {
        //         print!("{e:2} ");
        //     }
        //     println!();
        // }
        match matrix.last() {
            Some(row) => match row.last() {
                Some(v) => *v,
                None => ni.abs_diff(nj) as i32,
            },
            None => ni.abs_diff(nj) as i32,
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn min_distance_2(word1: String, word2: String) -> i32 {
        let mut chars1: Vec<char> = word1.chars().collect();
        let mut chars2: Vec<char> = word2.chars().collect();
        let mut ni = chars1.len() + 1;
        let mut nj = chars2.len() + 1;
        if nj > ni {
            std::mem::swap(&mut chars1, &mut chars2);
            std::mem::swap(&mut ni, &mut nj);
        }
        let mut distance = vec![0; nj];
        for i in 0..ni {
            let mut _distance = vec![0; nj];
            for j in 0..nj {
                let _i = i.checked_sub(1);
                let _j = j.checked_sub(1);
                let del = match _i {
                    Some(_i) => distance[j] + 1,
                    None => i32::MAX,
                };
                let ins = match _j {
                    Some(_j) => _distance[_j] + 1,
                    None => i32::MAX,
                };
                let rep = match (_i, _j) {
                    (Some(_i), Some(_j)) => {
                        if chars1[_i] == chars2[_j] {
                            distance[_j]
                        } else {
                            distance[_j] + 1
                        }
                    }
                    _ => i32::MAX,
                };
                let edit = match del.min(ins).min(rep) {
                    // i32::MAX => (chars1[i] != chars2[j]) as i32, // equal force replace!
                    i32::MAX => 0, // equal force replace!
                    v => v,
                };
                _distance[j] = edit;
            }
            distance = _distance;
        }
        match distance.last() {
            Some(v) => *v,
            None => ni.abs_diff(nj) as i32,
        }
    }
}

fn main() {
    for f in [Solution::min_distance_1, Solution::min_distance_2] {
        println!("===");
        assert_eq!(f("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(f("intention".to_string(), "execution".to_string()), 5);
        assert_eq!(f("".to_string(), "".to_string()), 0);
        assert_eq!(f("b".to_string(), "".to_string()), 1);
        assert_eq!(f("zool".to_string(), "zoog".to_string()), 1);
        assert_eq!(f("zool".to_string(), "zo".to_string()), 2);
        assert_eq!(f("a".to_string(), "b".to_string()), 1);
        assert_eq!(
            f(
                "zoologicoarchaeologist".to_string(),
                "zoogeologist".to_string()
            ),
            10
        );
        assert_eq!(f("".to_string(), "a".to_string()), 1);
        assert_eq!(f("eat".to_string(), "sea".to_string()), 2);
        assert_eq!(f("sea".to_string(), "eat".to_string()), 2);
    }
}
