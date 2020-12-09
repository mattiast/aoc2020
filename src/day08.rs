use std::io::{self, prelude::BufRead, BufReader};
use std::{collections::HashSet, fs::File};

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

    let mut acc: i32 = 0;
    let mut pc: usize = 0;

    let mut h: HashSet<usize> = HashSet::new();

    loop {
        h.insert(pc);
        println!("{} {}", pc, acc);
        match &v[pc] {
            (Op::Nop, _) => {
                pc += 1;
            }
            (Op::Acc, x) => {
                pc += 1;
                acc += x;
            }
            (Op::Jmp, x) => {
                pc = (pc as i32 + x) as usize;
            }
        }
        if h.contains(&pc) {
            return Ok(acc as i64);
        }
    }
}

pub fn part2() -> io::Result<i64> {
    todo!()
}
