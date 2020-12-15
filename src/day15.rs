use std::collections::HashMap;
use std::io;

pub fn part1(n: usize) -> io::Result<i64> {
    let mut v: HashMap<usize, usize> = [5, 1, 9, 18, 13, 8, 0]
        .iter()
        .enumerate()
        .map(|(i, j)| (*j, i))
        .collect();

    let mut last = 0;
    for i in v.len()..n - 1 {
        let jutska = v.get(&last).map_or(0, |j| i - j);
        v.insert(last, i);
        last = jutska;
    }
    println!("size {}", v.len());
    Ok(last as i64)
}
