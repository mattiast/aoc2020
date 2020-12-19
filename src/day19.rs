use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq)]
pub enum Token {
    A,
    B,
    Rule(usize),
}
#[derive(Debug, PartialEq)]
pub struct Rule {
    number: usize,
    pat: Vec<Vec<Token>>,
}

fn match_pat<'a>(
    pat: &'a Vec<Vec<Token>>,
    rules: &'a HashMap<usize, Rule>,
    input: &'a str,
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(
        pat.iter()
            .flat_map(move |term| match_term(term, rules, input)),
    )
}

fn match_term<'a>(
    term: &Vec<Token>,
    rules: &'a HashMap<usize, Rule>,
    input: &'a str,
) -> Option<&'a str> {
    let mut s = input;
    for t in term.iter() {
        s = match_token(t, rules, s)?;
    }
    Some(s)
}

fn match_token<'a>(t: &Token, rules: &'a HashMap<usize, Rule>, input: &'a str) -> Option<&'a str> {
    //
    if input == "" {
        return None;
    }
    let (eka, rest) = input.split_at(1);
    match t {
        Token::A => {
            if eka == "a" {
                Some(rest)
            } else {
                None
            }
        }
        Token::B => {
            if eka == "b" {
                Some(rest)
            } else {
                None
            }
        }
        Token::Rule(i) => {
            let r = &rules[i];
            match_pat(&r.pat, rules, input).next()
        }
    }
}

fn read_input() -> io::Result<Vec<Rule>> {
    let s = std::fs::read_to_string("data/day19.txt")?;
    let mut parts = s.split("\n\n");
    let mut r = vec![];
    let rules = parts.next().unwrap();
    for line in rules.split('\n') {
        let (_, l) = parsing::line(line).unwrap();
        r.push(l);
    }
    Ok(r)
}

pub fn part1() -> io::Result<i64> {
    let rules = read_input()?;
    let rules: HashMap<usize, Rule> = rules.into_iter().map(|r| (r.number, r)).collect();
    todo!()
}
pub fn part2() -> io::Result<i64> {
    todo!()
}

mod parsing {
    use super::{Rule, Token};
    use nom::bytes::complete::tag;
    use nom::combinator::map_res;
    use nom::{alt, bytes::complete::take_while1, do_parse, named, separated_list1, tag, IResult};
    fn parse_number(input: &str) -> IResult<&str, usize> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        })(input)
    }
    named!(r<&str, Token>, do_parse!(x1: parse_number >> (Token::Rule(x1))));
    named!(a<&str, Token>, do_parse!(x: tag!("\"a\"") >> (Token::A)));
    named!(b<&str, Token>, do_parse!(x: tag!("\"b\"") >> (Token::B)));
    named!(token<&str, Token>, alt!(r | a | b));
    named!(term<&str, Vec<Token>>, separated_list1!(tag(" "), token));
    named!(pat<&str, Vec<Vec<Token>>>, separated_list1!(tag(" | "), term));
    named!(pub line<&str, Rule>, do_parse!(number: parse_number >> tag!(": ") >> pat: pat >> (Rule { number, pat })));

    #[test]
    fn test_token() {
        assert_eq!(token("91"), Ok(("", Token::Rule(91))));
        assert_eq!(token("\"a\""), Ok(("", Token::A)));
    }

    #[test]
    fn test_line() {
        assert_eq!(term("91"), Ok(("", vec![Token::Rule(91)])));
        assert_eq!(
            term("91 50"),
            Ok(("", vec![Token::Rule(91), Token::Rule(50)]))
        );
        assert_eq!(
            pat("91 50"),
            Ok(("", vec![vec![Token::Rule(91), Token::Rule(50)]]))
        );
        assert_eq!(
            pat("91 50 | 77 115"),
            Ok((
                "",
                vec![
                    vec![Token::Rule(91), Token::Rule(50)],
                    vec![Token::Rule(77), Token::Rule(115)]
                ]
            ))
        );
        assert_eq!(
            line("57: 91 50 | 77 115"),
            Ok((
                "",
                Rule {
                    number: 57,
                    pat: vec![
                        vec![Token::Rule(91), Token::Rule(50)],
                        vec![Token::Rule(77), Token::Rule(115)]
                    ]
                }
            ))
        );
    }
}
