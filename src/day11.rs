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

fn directions() -> Vec<(isize, isize)> {
    vec![
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ]
}

fn act(i: (usize, usize), di: (isize, isize)) -> Option<(usize, usize)> {
    let j0 = Some((i.0 as isize) + di.0).filter(|&j| j >= 0 && j < 95)?;
    let j1 = Some((i.1 as isize) + di.1).filter(|&j| j >= 0 && j < 98)?;

    Some((j0 as usize, j1 as usize))
}

fn see_taken(
    i: (usize, usize),
    di: (isize, isize),
    a: &Array2<bool>,
    seats: &Array2<bool>,
) -> bool {
    let mut j = i;
    while let Some(j1) = act(j, di) {
        j = j1;
        if a[j] {
            return true;
        }
        if seats[j] {
            return false;
        }
    }
    return false;
}

fn step(a: &Array2<bool>, seats: &Array2<bool>) -> Array2<bool> {
    let mut b = a.clone();
    for (i, x) in b.indexed_iter_mut() {
        if !seats[i] {
            continue;
        }
        let mut count = 0usize;

        for di in directions() {
            if see_taken(i, di, a, seats) {
                count += 1;
            }
        }

        if !a[i] && count == 0 {
            *x = true;
        }
        if a[i] && count >= 5 {
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
