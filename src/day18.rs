use std::fs::File;
use std::io;
use std::io::{prelude::BufRead, BufReader};
pub fn part1() -> io::Result<i64> {
    let file = File::open("data/day18.txt")?;
    let reader = BufReader::new(file);

    let mut x = 0i64;
    for line in reader.lines() {
        let s = line.unwrap();
        let (_, r) = parsing::expr(&s).unwrap();
        x += r;
    }
    Ok(x)
}
pub fn part2() -> io::Result<i64> {
    todo!()
}
mod parsing {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::one_of,
        combinator::map_res,
        multi::many0,
        sequence::pair,
        IResult,
    };
    fn parse_number(input: &str) -> IResult<&str, i64> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        })(input)
    }
    fn parens<'a>(input: &'a str) -> IResult<&'a str, i64> {
        let (input, _) = tag("(")(input)?;
        let (input, n) = expr(input)?;
        let (input, _) = tag(")")(input)?;
        Ok((input, n))
    }
    pub fn term<'a>(input: &'a str) -> IResult<&'a str, i64> {
        alt((parens, parse_number))(input)
    }
    enum Op {
        Plus,
        Times,
    }

    fn operator<'a>(input: &'a str) -> IResult<&'a str, Op> {
        let (input, _) = tag(" ")(input)?;
        let (input, opc) = one_of("*+")(input)?;
        let (input, _) = tag(" ")(input)?;
        match opc {
            '+' => Ok((input, Op::Plus)),
            '*' => Ok((input, Op::Times)),
            _ => panic!("wrong op char"),
        }
    }
    pub fn expr<'a>(input: &'a str) -> IResult<&'a str, i64> {
        let (input, mut first) = term(input)?;
        let (input, rest) = many0(pair(operator, term))(input)?;

        for (op, x) in rest {
            first = match op {
                Op::Plus => first + x,
                Op::Times => first * x,
            }
        }

        Ok((input, first))
    }
    #[test]
    fn test_expr() {
        assert_eq!(expr("2 * 3 + (4 * 5)"), Ok(("", 26)));
        assert_eq!(expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(("", 437)));
        assert_eq!(
            expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(("", 12240))
        );
        assert_eq!(
            expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 13632))
        );
    }
}
