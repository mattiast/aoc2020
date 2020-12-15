use std::io;
pub fn part1(n: usize) -> io::Result<i64> {
    let mut v: Vec<usize> = vec![5, 1, 9, 18, 13, 8, 0];
    v.reserve(n);
    for i in v.len()..n {
        let last = v[i - 1];
        let jutska = (1..=i - 1).find(|j| v[i - 1 - j] == last).unwrap_or(0);
        v.push(jutska);
    }
    Ok(v[n - 1] as i64)
}
