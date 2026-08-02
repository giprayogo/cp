use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut k: usize = buf.trim().parse().unwrap();
    buf.clear();

    let mut results = Vec::new();
    while k != 0 {
        io::stdin().read_line(&mut buf).unwrap();
        let dp: Vec<i32> = buf
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        buf.clear();
        for _ in 0..k {
            io::stdin().read_line(&mut buf).unwrap();
            let p: Vec<i32> = buf
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            buf.clear();

            let q = (p[0] - dp[0], p[1] - dp[1]);
            if q.0 < 0 && q.1 < 0 {
                results.push("SO");
            } else if q.0 > 0 && q.1 > 0 {
                results.push("NE");
            } else if q.0 < 0 && q.1 > 0 {
                results.push("NO");
            } else if q.0 > 0 && q.1 < 0 {
                results.push("SE");
            } else {
                results.push("divisa");
            }
        }
        io::stdin().read_line(&mut buf).unwrap();
        k = buf.trim().parse().unwrap();
    }

    for result in results {
        println!("{}", result);
    }
}
