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
    let s = std::fs::read_to_string("data/day16.txt")?;

    let (_, stuff) = parsing::parse_file(&s).unwrap();
    // poss[i][j] = position i can be field number j
    let mut poss = [[true; 20]; 20];
    for ticket in stuff.nearby_tickets.iter() {
        if !is_valid_ticket(ticket, &stuff) {
            continue;
        }
        for (i, x) in ticket.iter().enumerate() {
            for (j, tavara) in stuff.fields.iter().enumerate() {
                if !is_valid(tavara, *x) {
                    poss[i][j] = false;
                }
            }
        }
    }
    while let Some(_) = reduce_poss(&mut poss) {}
    if false {
        for row in poss.iter() {
            let cs = row.iter().map(|&b| if b { '#' } else { ' ' });
            let s: String = cs.collect();
            println!("{}", s);
        }
    }
    let solution = find_solution(&poss);
    let ret = solution[0..6]
        .iter()
        .map(|&i| stuff.my_ticket[i])
        .product::<usize>();
    Ok(ret as i64)
}

fn reduce_poss(poss: &mut [[bool; 20]; 20]) -> Option<()> {
    let i = find_row(&poss)?;
    let j = poss[i]
        .iter()
        .enumerate()
        .filter(|(_, &x)| x)
        .next()
        .unwrap()
        .0;
    for ii in 0..20 {
        if ii != i {
            poss[ii][j] = false;
        }
    }

    Some(())
}

fn find_row(poss: &[[bool; 20]; 20]) -> Option<usize> {
    for (i, row) in poss.iter().enumerate() {
        let c = row.iter().filter(|&&x| x).count();
        if c == 1 {
            let j = row.iter().enumerate().filter(|(_, &x)| x).next().unwrap().0;
            if (0..20).filter(|&ii| poss[ii][j]).count() >= 2 {
                return Some(i);
            }
        }
    }
    None
}

fn find_solution(poss: &[[bool; 20]; 20]) -> [usize; 20] {
    let mut solution = [0; 20];

    for (i, row) in poss.iter().enumerate() {
        for (j, &x) in row.iter().enumerate() {
            if x {
                solution[j] = i;
            }
        }
    }

    solution
}

fn is_valid(t: &Tavara, x: usize) -> bool {
    (x >= t.r1.0 && x <= t.r1.1) || (x >= t.r2.0 && x <= t.r2.1)
}
fn is_valid_ticket(ticket: &Vec<usize>, stuff: &Stuff) -> bool {
    ticket
        .iter()
        .all(|&x| stuff.fields.iter().any(|tavara| is_valid(tavara, x)))
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
