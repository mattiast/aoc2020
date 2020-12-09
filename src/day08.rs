use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[derive(Debug)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

fn read_input() -> io::Result<Vec<(Op, i32)>> {
    let file = File::open("data/day08.txt")?;
    let reader = BufReader::new(file);

    let mut v = vec![];
    for line in reader.lines() {
        let line = line?;
        let op = match &line[0..3] {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            _ => panic!("wrong operation"),
        };
        let x: i32 = line[4..].parse().unwrap();
        v.push((op, x));
    }
    Ok(v)
}

pub fn part1() -> io::Result<i64> {
    let v = read_input()?;
    println!("{:?}", v);
    Ok(4)
}

pub fn part2() -> io::Result<i64> {
    todo!()
}
