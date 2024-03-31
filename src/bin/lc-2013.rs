use std::collections::HashMap;

struct DetectSquares {
    p: HashMap<(i32, i32), i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    fn new() -> Self {
        Self { p: HashMap::new() }
    }

    fn add(&mut self, point: Vec<i32>) {
        if let [x, y] = point[..] {
            self.p.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut c = 0;
        if let [x, y] = point[..] {
            let mut p_x = Vec::new();
            let mut p_y = Vec::new();
            for (_x, _y) in self.p.keys() {
                if *_x == x {
                    p_y.push((*_x, *_y));
                }
                if *_y == y {
                    p_x.push((*_x, *_y));
                }
            }
            for (_x, __y) in p_x.into_iter() {
                for (__x, _y) in p_y.iter() {
                    if let Some(v) = self.p.get(&(_x, *_y)) {
                        let dx = (_x - x).abs();
                        let dy = (_y - y).abs();
                        if dx == dy && dx != 0 {
                            c += v * self.p[&(_x, __y)] * self.p[&(*__x, *_y)];
                        }
                    }
                }
            }
        }
        c
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
fn main() {
    let mut d = DetectSquares::new();
    d.add(vec![3, 10]);
    d.add(vec![11, 2]);
    d.add(vec![3, 2]);
    assert_eq!(d.count(vec![11, 10]), 1);
    assert_eq!(d.count(vec![14, 8]), 0);
    d.add(vec![11, 2]);
    assert_eq!(d.count(vec![11, 10]), 2);
    d = DetectSquares::new();
    d.add(vec![5, 10]);
    d.add(vec![10, 5]);
    d.add(vec![10, 10]);
    assert_eq!(d.count(vec![5, 5]), 1);
    d.add(vec![3, 0]);
    d.add(vec![8, 0]);
    d.add(vec![8, 5]);
    assert_eq!(d.count(vec![3, 5]), 1);
    d.add(vec![9, 0]);
    d.add(vec![9, 8]);
    d.add(vec![1, 8]);
    assert_eq!(d.count(vec![1, 0]), 1);
    d.add(vec![0, 0]);
    d.add(vec![8, 0]);
    d.add(vec![8, 8]);
    assert_eq!(d.count(vec![0, 8]), 2);
}
