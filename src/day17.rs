use ndarray::prelude::*;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn read_input() -> io::Result<Array2<bool>> {
    let file = File::open("data/day17.txt")?;
    let reader = BufReader::new(file);
    let mut a = Array::from_elem((8, 8), false);

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
fn neighbors(i: (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    let (i0, i1, i2) = i;
    let mut v = vec![];
    for j0 in 0..20 {
        for j1 in 0..20 {
            for j2 in 0..20 {
                if j0 <= i0 + 1
                    && i0 <= j0 + 1
                    && j1 <= i1 + 1
                    && i1 <= j1 + 1
                    && j2 <= i2 + 1
                    && i2 <= j2 + 1
                    && (j0, j1, j2) != i
                {
                    v.push((j0, j1, j2));
                }
            }
        }
    }
    v
}

fn cycle(a: &mut Array3<bool>) {
    let b = a.clone();
    for (i, x) in a.indexed_iter_mut() {
        let mut c = 0;
        for j in neighbors(i) {
            if b[j] {
                c += 1;
            }
        }
        if !b[i] && c == 3 {
            *x = true;
        }
        if b[i] && !(c >= 2 && c <= 3) {
            *x = false;
        }
    }
}
pub fn part1() -> io::Result<i64> {
    let input = read_input()?;
    let mut a = Array::from_elem((20, 20, 20), false);
    for i in 0..8 {
        for j in 0..8 {
            a[(i + 6, j + 6, 10)] = input[(i, j)];
        }
    }
    for _ in 0..6 {
        cycle(&mut a);
    }
    let c = a.iter().filter(|&&x| x).count();
    Ok(c as i64)
}
pub fn part2() -> io::Result<i64> {
    todo!()
}
