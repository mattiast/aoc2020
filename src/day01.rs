use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn read_numbers() -> io::Result<Vec<usize>> {
    let file = File::open("data/day01.txt")?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let i: usize = line?.parse().unwrap();
            Ok(i)
        })
        .collect()
}

pub fn part1() -> io::Result<usize> {
    let numbers = read_numbers()?;

    let mut stuff: Vec<bool> = vec![false; 2021];
    for i in numbers {
        if i <= 2020 {
            stuff[i] = true;
        }
    }
    for i in 0..2021 {
        if stuff[i] && stuff[2020 - i] {
            return Ok(i * (2020 - i));
        }
    }
    Ok(5)
}

pub fn part2() -> io::Result<usize> {
    let numbers = read_numbers()?;

    let mut stuff: Vec<bool> = vec![false; 2021];
    for &i in numbers.iter() {
        if i <= 2020 {
            stuff[i] = true;
        }
    }
    for i in 0..200 {
        for j in i + 1..200 {
            let s = numbers[i] + numbers[j];
            if s <= 2020 && stuff[2020 - s] {
                return Ok(numbers[i] * numbers[j] * (2020 - s));
            }
        }
    }
    Ok(5)
}
