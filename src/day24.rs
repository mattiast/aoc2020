use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{prelude::BufRead, BufReader};

type Tile = (i32, i32, i32);

pub fn part1() -> io::Result<i64> {
    let file = File::open("data/day24.txt")?;
    let reader = BufReader::new(file);

    let mut h: HashSet<Tile> = HashSet::new();
    for line in reader.lines() {
        let line = line?;
        let (_, r) = parsing::tile(&line).unwrap();
        let t = add_steps(&r);
        println!("{:?}", t);
        if h.contains(&t) {
            h.remove(&t);
        } else {
            h.insert(t);
        }
    }
    Ok(h.len() as i64)
}

fn flipperi(h: HashSet<Tile>) -> HashSet<Tile> {
    let poss = h
        .iter()
        .cloned()
        .flat_map(|t| {
            let mut n = neighbors(t);
            n.push(t);
            n
        })
        .collect::<HashSet<_>>();

    poss.into_iter()
        .filter(|&t| {
            let n_active = neighbors(t).into_iter().filter(|n| h.contains(n)).count();
            (h.contains(&t) && (n_active == 1 || n_active == 2))
                || (!h.contains(&t) && n_active == 2)
        })
        .collect()
}

fn neighbors(t: Tile) -> Vec<Tile> {
    vec![
        add_step(t, (1, -1, 0)),
        add_step(t, (-1, 1, 0)),
        add_step(t, (0, 1, -1)),
        add_step(t, (0, -1, 1)),
        add_step(t, (1, 0, -1)),
        add_step(t, (-1, 0, 1)),
    ]
}

pub fn part2() -> io::Result<i64> {
    let file = File::open("data/day24.txt")?;
    let reader = BufReader::new(file);

    let mut h: HashSet<Tile> = HashSet::new();
    for line in reader.lines() {
        let line = line?;
        let (_, r) = parsing::tile(&line).unwrap();
        let t = add_steps(&r);
        if h.contains(&t) {
            h.remove(&t);
        } else {
            h.insert(t);
        }
    }
    for i in 0..100 {
        h = flipperi(h);
        println!("{} {:?}", i, h.len());
    }
    Ok(h.len() as i64)
}

fn add_step(t1: Tile, t2: Tile) -> Tile {
    let (a1, b1, c1) = t1;
    let (a2, b2, c2) = t2;
    (a1 + a2, b1 + b2, c1 + c2)
}

fn add_steps(ts: &[Tile]) -> Tile {
    ts.iter().fold((0, 0, 0), |t1, &t2| add_step(t1, t2))
}

mod parsing {
    use super::Tile;
    use nom::{alt, complete, do_parse, many1, named, tag};

    named!(e<&str, Tile>, do_parse!(complete!(tag!("e")) >> ((1,-1,0))));
    named!(w<&str, Tile>, do_parse!(complete!(tag!("w")) >> ((-1,1,0))));
    named!(nw<&str, Tile>, do_parse!(complete!(tag!("nw")) >> ((0,1,-1))));
    named!(ne<&str, Tile>, do_parse!(complete!(tag!("se")) >> ((0,-1,1))));
    named!(sw<&str, Tile>, do_parse!(complete!(tag!("sw")) >> ((-1,0,1))));
    named!(se<&str, Tile>, do_parse!(complete!(tag!("ne")) >> ((1,0,-1))));

    named!(step<&str, Tile>, alt!(e | w | nw | ne | sw | se));
    named!(pub tile<&str, Vec<Tile>>, many1!(step));
}
