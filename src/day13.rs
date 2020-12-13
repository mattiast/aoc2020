use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn read_input() -> io::Result<(i128, Vec<Option<i128>>)> {
    let file = File::open("data/day13.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let line = lines.next().unwrap()?;
    let x: i128 = line.parse().unwrap();

    let line = lines.next().unwrap()?;
    let v: Vec<Option<i128>> = line
        .split(',')
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect();

    Ok((x, v))
}

pub fn part1() -> io::Result<i64> {
    let (x, v) = read_input()?;

    let i = v
        .into_iter()
        .filter_map(|i| i)
        .min_by_key(|i| i - x % i)
        .unwrap();

    let ret = i * (i - x % i);
    Ok(ret as i64)
}
pub fn part2() -> io::Result<i64> {
    todo!()
}
