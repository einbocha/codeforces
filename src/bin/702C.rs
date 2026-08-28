use std::io;

fn main() {
    let mut line = String::new();

    // first line
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let nc: usize = iter.next().unwrap().parse().unwrap();
    let mt: usize = iter.next().unwrap().parse().unwrap();

    // second line
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let c: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // third line
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let t: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(nc, c.len());
    assert_eq!(mt, t.len());

    let mut res: i32 = 0;

    if t.len() == 1 {
        let d1: i32 = (t[0] - c[0]).abs();
        let d2: i32 = (t[0] - c[c.len() - 1]).abs();
        res = d1.max(d2);
        println!("{}", res);
        return;
    }

    let mut p: usize = 0;

    for i in 0..(t.len() - 1) {
        let first: i32 = t[i];
        let second: i32 = t[i + 1];
        while p < c.len() && (c[p] < second || i + 2 == t.len()) {
            let city: i32 = c[p];
            let d1: i32 = (first - city).abs();
            let d2: i32 = (second - city).abs();
            res = res.max(d1.min(d2));
            p += 1;
        }
    }

    println!("{}", res);
}