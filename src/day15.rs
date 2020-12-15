use std::io;

pub fn part1(n: usize) -> io::Result<i64> {
    let input: Vec<usize> = vec![5, 1, 9, 18, 13, 8, 0];
    let mut v: Vec<Option<usize>> = vec![None; n];

    for (i, &x) in input.iter().enumerate() {
        v[x] = Some(i);
    }

    let mut last = 0;
    for i in input.len()..n - 1 {
        let jutska = v[last].map_or(0, |j| i - j);
        v[last] = Some(i);
        last = jutska;
    }
    Ok(last as i64)
}
