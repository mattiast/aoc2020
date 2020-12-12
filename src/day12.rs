use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

fn read_input() -> io::Result<Vec<(Action, i32)>> {
    let file = File::open("data/day12.txt")?;
    let reader = BufReader::new(file);

    let mut v = vec![];
    for line in reader.lines() {
        let line = line?;
        let op = match &line[0..1] {
            "N" => Action::N,
            "S" => Action::S,
            "E" => Action::E,
            "W" => Action::W,
            "L" => Action::L,
            "R" => Action::R,
            "F" => Action::F,
            _ => panic!("wrong action"),
        };
        let mut x: i32 = line[1..].parse().unwrap();
        if op == Action::L || op == Action::R {
            assert!(x % 90 == 0);
            x = x / 90;
        }
        v.push((op, x));
    }
    Ok(v)
}

#[derive(Debug)]
struct State {
    loc: (i32, i32),
    dir: (i32, i32),
}

fn step(p: State, (action, length): (Action, i32)) -> State {
    let (x, y) = p.loc;
    let (dx, dy) = p.dir;
    let loc = match action {
        Action::N => (x, y + length),
        Action::S => (x, y - length),
        Action::E => (x + length, y),
        Action::W => (x - length, y),
        Action::F => (x + length * dx, y + length * dy),
        Action::L => (x, y),
        Action::R => (x, y),
    };
    let (mut dx, mut dy) = p.dir;
    if action == Action::L {
        let d = turn((dx, dy), length);
        dx = d.0;
        dy = d.1;
    }
    if action == Action::R {
        let d = turn((dx, dy), 4 - length);
        dx = d.0;
        dy = d.1;
    }
    let dir = (dx, dy);
    State { loc, dir }
}

pub fn part1() -> io::Result<i64> {
    let input = read_input()?;
    let mut s = State {
        loc: (0, 0),
        dir: (1, 0),
    };
    for i in input {
        s = step(s, i);
    }
    let (x, y) = s.loc;
    Ok((x.abs() + y.abs()) as i64)
}

fn turn((mut dx, mut dy): (i32, i32), mut n: i32) -> (i32, i32) {
    n = n % 4;
    for _ in 0..n {
        let ff = dx;
        dx = -dy;
        dy = ff;
    }
    (dx, dy)
}

fn step2(p: State, (action, length): (Action, i32)) -> State {
    let (x, y) = p.loc;
    let (dx, dy) = p.dir;
    let loc = if let Action::F = action {
        (x + length * dx, y + length * dy)
    } else {
        p.loc
    };
    let mut dir = p.dir;
    dir = match action {
        Action::L => turn(dir, length),
        Action::R => turn(dir, 4 - length),
        Action::N => (dir.0, dir.1 + length),
        Action::S => (dir.0, dir.1 - length),
        Action::E => (dir.0 + length, dir.1),
        Action::W => (dir.0 - length, dir.1),
        Action::F => dir,
    };
    State { loc, dir }
}

pub fn part2() -> io::Result<i64> {
    let input = read_input()?;
    let mut s = State {
        loc: (0, 0),
        dir: (10, 1),
    };
    for i in input {
        s = step2(s, i);
    }
    let (x, y) = s.loc;
    Ok((x.abs() + y.abs()) as i64)
}
