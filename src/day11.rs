use ndarray::prelude::*;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn read_input() -> io::Result<Array2<bool>> {
    let file = File::open("data/day11.txt")?;
    let reader = BufReader::new(file);
    let mut a = Array::from_elem((95, 98), false);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        for (j, c) in line.chars().enumerate() {
            if c == 'L' {
                a[[i, j]] = true;
            }
        }
    }
    Ok(a)
}

fn step(a: &Array2<bool>, seats: &Array2<bool>) -> Array2<bool> {
    let mut b = a.clone();
    for ((i, j), x) in b.indexed_iter_mut() {
        if !seats[(i, j)] {
            continue;
        }
        let mut count = 0usize;

        if i > 0 && j > 0 && a[(i - 1, j - 1)] {
            count += 1;
        }
        if i > 0 && a[(i - 1, j)] {
            count += 1;
        }
        if j > 0 && a[(i, j - 1)] {
            count += 1;
        }
        if i < 94 && j < 97 && a[(i + 1, j + 1)] {
            count += 1;
        }
        if i < 94 && a[(i + 1, j)] {
            count += 1;
        }
        if j < 97 && a[(i, j + 1)] {
            count += 1;
        }
        if i < 94 && j > 0 && a[(i + 1, j - 1)] {
            count += 1;
        }
        if i > 0 && j < 97 && a[(i - 1, j + 1)] {
            count += 1;
        }

        if !a[(i, j)] && count == 0 {
            *x = true;
        }
        if a[(i, j)] && count >= 4 {
            *x = false;
        }
    }
    b
}

pub fn part1() -> io::Result<i64> {
    let seats = read_input()?;

    let mut a = Array2::from_elem((95, 98), false);
    loop {
        let b = step(&a, &seats);
        if b == a {
            break;
        }
        a = b;
    }

    let x = a.iter().filter(|x| **x).count();

    Ok(x as i64)
}

pub fn part2() -> io::Result<i64> {
    todo!()
}
