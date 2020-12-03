use ndarray::prelude::*;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn read_input() -> io::Result<Array2<bool>> {
    let file = File::open("data/day03.txt")?;
    let reader = BufReader::new(file);
    let mut a = Array::from_elem((323, 31), false);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                a[[i, j]] = true;
            }
        }
    }
    Ok(a)
}

fn jutska(a: &Array2<bool>, step: usize) -> i64 {
    let mut x = 0;
    for i in 0..323 {
        let j = (step * i) % 31;
        if a[[i, j]] {
            x += 1;
        }
    }
    x
}

fn jutska2(a: &Array2<bool>) -> i64 {
    let mut x = 0;
    for i in 0..=323 / 2 {
        let j = i % 31;
        if a[[2 * i, j]] {
            x += 1;
        }
    }
    x
}

pub fn part1() -> io::Result<i64> {
    let a = read_input()?;
    let x = jutska(&a, 3);
    Ok(x)
}
pub fn part2() -> io::Result<i64> {
    let a = read_input()?;
    let x = jutska(&a, 1) * jutska(&a, 3) * jutska(&a, 5) * jutska(&a, 7) * (jutska2(&a));
    Ok(x)
}
