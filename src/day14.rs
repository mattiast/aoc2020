use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Mask {
    xmask: u64,
    value: u64,
}

fn use_mask(m: Mask, x: u64) -> u64 {
    (m.xmask & x) + m.value
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Mask(Mask),
    Mem { addr: u64, value: u64 },
}

pub fn read_input() -> io::Result<Vec<Statement>> {
    let file = File::open("data/day14.txt")?;
    let reader = BufReader::new(file);

    let mut v = vec![];
    for line in reader.lines() {
        let line = line?;
        let (_, stmt) = parsing::parse_line(&line).unwrap();
        v.push(stmt);
    }
    Ok(v)
}

pub fn part1() -> io::Result<i64> {
    let ss = read_input()?;
    let mut mask = Mask { xmask: 0, value: 0 };
    let mut memory = HashMap::<u64, u64>::new();

    for stmt in ss {
        //
        match stmt {
            Statement::Mem { addr, value } => {
                memory.insert(addr, use_mask(mask, value));
            }
            Statement::Mask(new_mask) => {
                mask = new_mask;
            }
        }
    }
    Ok(memory.into_iter().map(|x| x.1).sum::<u64>() as i64)
}
pub fn part2() -> io::Result<i64> {
    todo!()
}

mod parsing {
    use super::{Mask, Statement};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::satisfy,
        combinator::map_res,
        multi::many_m_n,
        IResult,
    };
    fn parse_number(input: &str) -> IResult<&str, u64> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        })(input)
    }
    fn xbit(input: &str) -> IResult<&str, Option<bool>> {
        let (input, x) = satisfy(|c| c == 'X' || c == '0' || c == '1')(input)?;
        let ret = match x {
            'X' => None,
            '0' => Some(false),
            '1' => Some(true),
            _ => panic!("wrong char"),
        };
        Ok((input, ret))
    }
    fn mask<'a>(input: &'a str) -> IResult<&'a str, Mask> {
        let (input, xbits) = many_m_n(36, 36, xbit)(input)?;
        let mut xmask = 0u64;
        let mut value = 0u64;
        for xbit in xbits {
            xmask <<= 1;
            value <<= 1;
            match xbit {
                None => {
                    xmask += 1;
                }
                Some(true) => {
                    value += 1;
                }
                Some(false) => {}
            }
        }
        Ok((input, Mask { xmask, value }))
    }
    fn set_mask<'a>(input: &'a str) -> IResult<&'a str, Statement> {
        let (input, _) = tag("mask = ")(input)?;
        let (input, m) = mask(input)?;
        Ok((input, Statement::Mask(m)))
    }
    fn set_mem<'a>(input: &'a str) -> IResult<&'a str, Statement> {
        let (input, _) = tag("mem[")(input)?;
        let (input, addr) = parse_number(input)?;
        let (input, _) = tag("] = ")(input)?;
        let (input, value) = parse_number(input)?;
        Ok((input, Statement::Mem { addr, value }))
    }
    pub fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Statement> {
        alt((set_mask, set_mem))(input)
    }

    #[test]
    fn test_line() {
        let x = parse_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(
            x,
            Ok((
                "",
                Statement::Mask(Mask {
                    xmask: 68719476669,
                    value: 64,
                })
            ))
        );

        let x = parse_line("mem[7] = 101");
        assert_eq!(
            x,
            Ok((
                "",
                Statement::Mem {
                    addr: 7,
                    value: 101
                }
            ))
        );
    }
}
