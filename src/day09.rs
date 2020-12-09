use std::io::{self, prelude::BufRead, BufReader};
use std::{collections::HashSet, fs::File};

pub fn read_input() -> io::Result<Vec<u64>> {
    let file = File::open("data/day09.txt")?;
    let reader = BufReader::new(file);

    let mut v = vec![];
    for line in reader.lines() {
        let line = line?;
        let x: u64 = line.parse().unwrap();
        v.push(x);
    }
    Ok(v)
}

pub fn part1() -> io::Result<i64> {
    let v = read_input()?;

    for n in 25..v.len() {
        let mut h: HashSet<u64> = HashSet::with_capacity(25 * 24 / 2);
        for i in n - 25..n {
            for j in i + 1..n {
                h.insert(v[i] + v[j]);
            }
        }
        let x = v[n];
        if !h.contains(&x) {
            return Ok(x as i64);
        }
    }

    Ok(5)
}

pub fn part2() -> io::Result<i64> {
    let v = read_input()?;
    let invalid_number = part1()? as u64;

    let cumsum: Vec<_> = v
        .iter()
        .scan(0u64, |x, y| {
            *x += y;
            Some(*x)
        })
        .collect();
    // cumsum won't include 0 at the beginning, let's just hope first item isn't included in the slice
    for i in 1..v.len() {
        for j in i + 1..v.len() {
            if cumsum[j] - cumsum[i - 1] == invalid_number {
                let slice = &v[i..j + 1];
                let max = slice.iter().max().unwrap();
                let min = slice.iter().min().unwrap();
                return Ok((max + min) as i64);
            }
        }
    }

    Ok(4)
}
