use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, prelude::BufRead, BufReader},
};
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

fn targets(i: usize, v: &[(Op, i32)]) -> Vec<(usize, i32, u32)> {
    if i == 654 {
        return vec![];
    }
    match v[i] {
        (Op::Acc, x) => vec![(i + 1, x, 0)],
        (Op::Jmp, j) => {
            let mut ts = Vec::with_capacity(2);
            let x = i as i32 + j;
            if x >= 0 && x <= 654 {
                ts.push((x as usize, 0, 0));
            }
            if j != 1 {
                ts.push((i + 1, 0, 1));
            }
            ts
        }
        (Op::Nop, j) => {
            let mut ts = Vec::with_capacity(2);
            ts.push((i + 1, 0, 0));

            let x = i as i32 + j;
            if x >= 0 && x <= 654 && x != (i + 1) as i32 {
                ts.push((x as usize, 0, 1));
            }
            ts
        }
    }
}

pub fn part2() -> io::Result<i64> {
    let v = read_input()?;

    let mut visited: HashSet<usize> = HashSet::new();
    let mut seen: BinaryHeap<(Reverse<u32>, i32, usize)> = BinaryHeap::new();
    seen.push((Reverse(0u32), 0i32, 0usize));

    while let Some((Reverse(d), acc, i)) = seen.pop() {
        if d >= 2 {
            break;
        }
        if i == 654 {
            return Ok(acc as i64);
        }
        if visited.contains(&i) {
            continue;
        }
        visited.insert(i);
        let ts = targets(i, &v);
        for (j, x, dd) in ts {
            seen.push((Reverse(d + dd), acc + x, j));
        }
    }

    Ok(4)
}

#[test]
fn test_pair_ord() {
    let x: (i32, usize) = (-1, 3);
    let y: (i32, usize) = (0, 1);
    assert!(y > x);
}
