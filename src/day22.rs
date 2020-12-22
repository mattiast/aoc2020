use color_eyre::{eyre::eyre, Result};
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{prelude::BufRead, BufReader};

struct Gamestate {
    p1: VecDeque<u32>,
    p2: VecDeque<u32>,
}

fn ended(gs: &Gamestate) -> Option<bool> {
    if gs.p1.is_empty() {
        Some(false)
    } else if gs.p2.is_empty() {
        Some(true)
    } else {
        None
    }
}

fn step(gs: &mut Gamestate) -> Option<()> {
    let x1 = gs.p1.pop_front()?;
    let x2 = gs.p2.pop_front()?;
    if x1 > x2 {
        gs.p1.push_back(x1);
        gs.p1.push_back(x2);
    } else {
        gs.p2.push_back(x2);
        gs.p2.push_back(x1);
    }
    Some(())
}

pub fn read_input() -> Result<(Vec<u32>, Vec<u32>)> {
    let file = File::open("data/day22.txt")?;
    let mut lines = BufReader::new(file).lines();

    let mut v1 = Vec::new();
    lines.next().ok_or(eyre!("ran out"))??;
    for _ in 0..25 {
        let line = lines.next().ok_or(eyre!("ran out"))??;
        v1.push(line.parse::<u32>()?);
    }
    lines.next().ok_or(eyre!("ran out"))??;
    lines.next().ok_or(eyre!("ran out"))??;
    let mut v2 = Vec::new();
    for _ in 0..25 {
        let line = lines.next().ok_or(eyre!("ran out"))??;
        v2.push(line.parse::<u32>()?);
    }
    Ok((v1, v2))
}

pub fn part1() -> Result<i64> {
    let (p1, p2) = read_input()?;
    let mut gs = Gamestate {
        p1: p1.into(),
        p2: p2.into(),
    };
    let winner: bool;
    loop {
        if let Some(w) = ended(&gs) {
            winner = w;
            break;
        }
        step(&mut gs);
    }
    let deck = if winner { &gs.p1 } else { &gs.p2 };
    let r = deck
        .iter()
        .zip(0..50)
        .map(|(x, y)| x * (50 - y))
        .sum::<u32>();
    Ok(r as i64)
}
pub fn part2() -> io::Result<i64> {
    todo!()
}
