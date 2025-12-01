use std::collections::{BTreeSet, HashMap, HashSet};
use std::io;

#[derive(Debug, PartialEq)]
pub struct Rule<'a> {
    color: &'a str,
    others: Vec<(&'a str, usize)>,
}

pub fn part1() -> io::Result<usize> {
    let s = std::fs::read_to_string("data/day07.txt")?;

    let (_, rules) = parsing::parse_file(&s).unwrap();

    let mut h: HashMap<&str, HashSet<&str>> = HashMap::new();
    for r in rules {
        for (s, _) in r.others {
            let e = h.entry(s).or_default();
            e.insert(r.color);
        }
    }

    let mut seen: BTreeSet<&str> = ["shiny gold"].iter().cloned().collect();
    let mut visited: HashSet<&str> = HashSet::new();
    while let Some(&x) = seen.iter().next() {
        seen.remove(x);
        if visited.contains(x) {
            continue;
        }
        visited.insert(x);
        if let Some(s) = h.get(x) {
            for y in s.iter() {
                seen.insert(y);
            }
        }
    }
    Ok(visited.len() - 1)
}

pub fn part2() -> io::Result<usize> {
    let s = std::fs::read_to_string("data/day07.txt")?;

    let (_, rules) = parsing::parse_file(&s).unwrap();

    let h: HashMap<_, _> = rules.into_iter().map(|r| (r.color, r.others)).collect();

    let mut cache: HashMap<&str, usize> = HashMap::new();
    Ok(calculate("shiny gold", &h, &mut cache) - 1)
}

fn calculate<'a>(
    color: &'a str,
    rules: &HashMap<&'a str, Vec<(&'a str, usize)>>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(x) = cache.get(color) {
        return *x;
    }
    let contents = &rules[color];
    let x: usize = contents
        .iter()
        .map(|(c, n)| n * calculate(c, rules, cache))
        .sum();
    let x = x + 1;
    cache.insert(color, x);
    x
}

mod parsing {
    use super::Rule;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until, take_while1},
        combinator::map_res,
        multi::separated_list1,
        IResult, Parser,
    };
    fn parse_number(input: &str) -> IResult<&str, usize> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        }).parse(input)
    }
    fn no_bags<'a>(input: &'a str) -> IResult<&'a str, Vec<(&'a str, usize)>> {
        let (input, _) = tag("no other bags").parse(input)?;
        Ok((input, vec![]))
    }
    fn bag_count<'a>(input: &'a str) -> IResult<&'a str, (&'a str, usize)> {
        let (input, n) = parse_number(input)?;
        let (input, _) = tag(" ").parse(input)?;
        let (input, color) = take_until(" bag").parse(input)?;
        let mut bag = if n == 1 { tag(" bag") } else { tag(" bags") };
        let (input, _) = bag.parse(input)?;
        Ok((input, (color, n)))
    }
    fn bag_counts1<'a>(input: &'a str) -> IResult<&'a str, Vec<(&'a str, usize)>> {
        separated_list1(tag(", "), bag_count).parse(input)
    }
    fn bag_counts<'a>(input: &'a str) -> IResult<&'a str, Vec<(&'a str, usize)>> {
        alt((no_bags, bag_counts1)).parse(input)
    }
    pub fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Rule<'a>> {
        let (input, color) = take_until(" bags contain ").parse(input)?;
        let (input, _) = tag(" bags contain ").parse(input)?;
        let (input, others) = bag_counts(input)?;
        let (input, _) = tag(".").parse(input)?;
        Ok((input, Rule { color, others }))
    }
    pub fn parse_file<'a>(input: &'a str) -> IResult<&'a str, Vec<Rule<'a>>> {
        separated_list1(tag("\n"), parse_line).parse(input)
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
