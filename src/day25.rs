use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn read_input() -> io::Result<(u64, u64)> {
    let file = File::open("data/day25.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let line = lines.next().unwrap()?;
    let x: u64 = line.parse().unwrap();
    let line = lines.next().unwrap()?;
    let y: u64 = line.parse().unwrap();

    Ok((x, y))
}

pub fn part1() -> io::Result<i64> {
    let (x, y) = read_input()?;
    let p = 20201227u64;
    let dl: Vec<u64> = {
        let mut dl = vec![0; 20201227];
        let mut x = 1u64;
        for i in 0..p - 1 {
            dl[x as usize] = i;
            x = (x * 7) % p;
        }
        dl
    };
    let (a, b) = (dl[x as usize], dl[y as usize]);

    let key = {
        let mut k = 1;
        for _ in 0..a {
            k *= y;
            k %= p;
        }
        k
    };

    println!("{} {}", a, b);
    Ok(key as i64)
}
pub fn part2() -> io::Result<i64> {
    todo!()
}
