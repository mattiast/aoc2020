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
mod parsing {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        combinator::map_res,
        multi::separated_list1,
        IResult, Parser,
    };
    fn parse_number(input: &str) -> IResult<&str, i64> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        }).parse(input)
    }
    fn parens<'a>(input: &'a str) -> IResult<&'a str, i64> {
        let (input, _) = tag("(").parse(input)?;
        let (input, n) = expr(input)?;
        let (input, _) = tag(")").parse(input)?;
        Ok((input, n))
    }
    pub fn term<'a>(input: &'a str) -> IResult<&'a str, i64> {
        alt((parens, parse_number)).parse(input)
    }
    pub fn term1<'a>(input: &'a str) -> IResult<&'a str, i64> {
        let (input, v) = separated_list1(tag(" + "), term).parse(input)?;

        Ok((input, v.iter().sum()))
    }
    pub fn expr<'a>(input: &'a str) -> IResult<&'a str, i64> {
        let (input, v) = separated_list1(tag(" * "), term1).parse(input)?;

        Ok((input, v.iter().product()))
    }
    #[test]
    fn test_expr() {
        assert_eq!(expr("1 + (2 * 3) + (4 * (5 + 6))"), Ok(("", 51)));
        assert_eq!(expr("2 * 3 + (4 * 5)"), Ok(("", 46)));
        assert_eq!(expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(("", 1445)));
        assert_eq!(
            expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(("", 669060))
        );
        assert_eq!(
            expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 23340))
        );
    }
}
