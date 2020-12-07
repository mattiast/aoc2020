use std::fs::File;
use std::io;
use std::io::{prelude::BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub struct Rule<'a> {
    color: &'a str,
    others: Vec<(&'a str, usize)>,
}

pub fn part1() -> io::Result<usize> {
    let file = File::open("data/day07.txt")?;
    let reader = BufReader::new(file);

    let mut x = 0usize;
    for line in reader.lines() {
        let s = line.unwrap();
        let (_, r) = parsing::parse_line(&s).unwrap();
        println!("{:?}", r);
        x += 1;
    }
    Ok(x)
}

mod parsing {
    use super::Rule;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until, take_while1},
        combinator::map_res,
        multi::separated_list1,
        IResult,
    };
    fn parse_number(input: &str) -> IResult<&str, usize> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        })(input)
    }
    fn no_bags<'a>(input: &'a str) -> IResult<&'a str, Vec<(&'a str, usize)>> {
        let (input, _) = tag("no other bags")(input)?;
        Ok((input, vec![]))
    }
    fn bag_count<'a>(input: &'a str) -> IResult<&'a str, (&'a str, usize)> {
        let (input, n) = parse_number(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, color) = take_until(" bag")(input)?;
        let bag = if n == 1 { tag(" bag") } else { tag(" bags") };
        let (input, _) = bag(input)?;
        Ok((input, (color, n)))
    }
    fn bag_counts1<'a>(input: &'a str) -> IResult<&'a str, Vec<(&'a str, usize)>> {
        separated_list1(tag(", "), bag_count)(input)
    }
    fn bag_counts<'a>(input: &'a str) -> IResult<&'a str, Vec<(&'a str, usize)>> {
        alt((no_bags, bag_counts1))(input)
    }
    pub fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Rule<'a>> {
        let (input, color) = take_until(" bags contain ")(input)?;
        let (input, _) = tag(" bags contain ")(input)?;
        let (input, others) = bag_counts(input)?;
        let (input, _) = tag(".")(input)?;
        Ok((input, Rule { color, others }))
    }

    #[test]
    fn test_bc() {
        let x = bag_count("2 shiny gold bags");
        assert_eq!(x, Ok(("", ("shiny gold", 2))));

        let x = bag_count("1 dark olive bag");
        assert_eq!(x, Ok(("", ("dark olive", 1))));
    }

    #[test]
    fn test_line() {
        let x = parse_line("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.");
        assert_eq!(
            x,
            Ok((
                "",
                Rule {
                    color: "shiny gold",
                    others: vec![("dark olive", 1), ("vibrant plum", 2)]
                }
            ))
        );
        let x = parse_line("faded blue bags contain no other bags.");
        assert_eq!(
            x,
            Ok((
                "",
                Rule {
                    color: "faded blue",
                    others: vec![]
                }
            ))
        );
    }
}
