use std::io;

pub fn part1() -> io::Result<i64> {
    let s = std::fs::read_to_string("data/day16.txt")?;

    let (_, stuff) = parsing::parse_file(&s).unwrap();
    let mut sima = 0usize;
    for ticket in stuff.nearby_tickets.iter() {
        for x in ticket {
            if !stuff.fields.iter().any(|t| is_valid(t, *x)) {
                sima += x;
            }
        }
    }
    Ok(sima as i64)
}
pub fn part2() -> io::Result<i64> {
    todo!()
}

fn is_valid(t: &Tavara, x: usize) -> bool {
    (x >= t.r1.0 && x <= t.r1.1) || (x >= t.r2.0 && x <= t.r2.1)
}

#[derive(Debug, PartialEq)]
pub struct Tavara {
    r1: (usize, usize),
    r2: (usize, usize),
}

#[derive(Debug, PartialEq)]
pub struct Stuff {
    fields: Vec<Tavara>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

mod parsing {
    use super::*;
    use nom::{
        bytes::complete::{tag, take_while1},
        combinator::map_res,
        multi::separated_list1,
        IResult,
    };
    fn parse_number(input: &str) -> IResult<&str, usize> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        })(input)
    }
    fn parse_range(input: &str) -> IResult<&str, (usize, usize)> {
        let (input, a) = parse_number(input)?;
        let (input, _) = tag("-")(input)?;
        let (input, b) = parse_number(input)?;
        Ok((input, (a, b)))
    }
    fn parse_field_line<'a>(input: &'a str) -> IResult<&'a str, Tavara> {
        let (input, _) = take_while1(|c: char| c.is_alphabetic() || c == ' ')(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, r1) = parse_range(input)?;
        let (input, _) = tag(" or ")(input)?;
        let (input, r2) = parse_range(input)?;

        Ok((input, Tavara { r1, r2 }))
    }
    fn parse_ticket(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, v) = separated_list1(tag(","), parse_number)(input)?;
        Ok((input, v))
    }

    pub fn parse_file(input: &str) -> IResult<&str, Stuff> {
        let (input, fields) = separated_list1(tag("\n"), parse_field_line)(input)?;
        let (input, _) = tag("\n\nyour ticket:\n")(input)?;
        let (input, my_ticket) = parse_ticket(input)?;
        let (input, _) = tag("\n\nnearby tickets:\n")(input)?;
        let (input, nearby_tickets) = separated_list1(tag("\n"), parse_ticket)(input)?;
        Ok((
            input,
            Stuff {
                fields,
                my_ticket,
                nearby_tickets,
            },
        ))
    }

    #[test]
    fn test_field_line() {
        assert_eq!(
            parse_field_line("arrival track: 48-377 or 401-964"),
            Ok((
                "",
                Tavara {
                    r1: (48, 377),
                    r2: (401, 964)
                }
            ))
        );
    }
}
