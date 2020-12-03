use std::fs::File;
use std::io;
use std::io::{prelude::BufRead, BufReader};

pub struct Password<'a> {
    c: char,
    lo: usize,
    hi: usize,
    passu: &'a str,
}

fn check<'a>(p: Password<'a>) -> bool {
    let n_jutska = p.passu.chars().filter(|&c| c == p.c).count();
    n_jutska <= p.hi && n_jutska >= p.lo
}

fn check2<'a>(p: Password<'a>) -> bool {
    let charvec: Vec<_> = p.passu.chars().collect();
    (charvec[p.lo - 1] == p.c) ^ (charvec[p.hi - 1] == p.c)
}

pub fn part1() -> io::Result<usize> {
    let file = File::open("data/day02.txt")?;
    let reader = BufReader::new(file);

    let mut x = 0usize;
    for line in reader.lines() {
        let s = line.unwrap();
        let (_, r) = parsing::parse_password(&s).unwrap();
        if check(r) {
            x += 1;
        }
    }
    Ok(x)
}
pub fn part2() -> io::Result<usize> {
    let file = File::open("data/day02.txt")?;
    let reader = BufReader::new(file);

    let mut x = 0usize;
    for line in reader.lines() {
        let s = line.unwrap();
        let (_, r) = parsing::parse_password(&s).unwrap();
        if check2(r) {
            x += 1;
        }
    }
    Ok(x)
}

mod parsing {
    use super::Password;
    use nom::{
        bytes::complete::{tag, take_while1},
        character::complete::satisfy,
        combinator::map_res,
        IResult,
    };
    fn parse_number(input: &str) -> IResult<&str, usize> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        })(input)
    }

    pub fn parse_password<'a>(input: &'a str) -> IResult<&'a str, Password<'a>> {
        let (input, lo) = parse_number(input)?;
        let (input, _) = tag("-")(input)?;
        let (input, hi) = parse_number(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, c) = satisfy(|c| c.is_alphabetic())(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, passu) = take_while1(|c: char| c.is_alphabetic())(input)?;
        Ok((input, Password { lo, hi, c, passu }))
    }
}
