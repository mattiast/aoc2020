use std::collections::HashSet;
use std::io;

pub fn part1() -> io::Result<usize> {
    let allfds: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .into_iter()
        .collect();
    let input = std::fs::read_to_string("data/day04.txt")?;
    let (_, pss) = parsing::parse_passports(&input).unwrap();

    let mut x = 0;
    for ps in pss {
        let mut fds: HashSet<_> = ps.into_iter().collect();
        fds.insert("cid");
        if fds == allfds {
            x += 1;
        }
    }

    Ok(x)
}
#[derive(Debug, PartialEq)]
enum HUnit {
    In,
    Cm,
}
#[derive(Debug, PartialEq)]
enum Ecl {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

#[derive(Debug, PartialEq)]
enum Field {
    Byr(u32),
    Iyr(u32),
    Eyr(u32),
    Hgt(u32, HUnit),
    Hcl(u8, u8, u8),
    Ecl(Ecl),
    Pid([u8; 9]),
    Cid,
}

mod parsing {
    use super::Field;
    use nom::combinator::map_res;
    use nom::{
        alt,
        bytes::complete::{tag, take, take_while1},
        character::complete::satisfy,
        do_parse, many_m_n,
        multi::separated_list1,
        named, tag, take, take_while1, verify, IResult,
    };
    fn parse_number(input: &str) -> IResult<&str, usize> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
            input.parse()
        })(input)
    }
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.

    named!(byr<&str, Field>, do_parse!(tag!("byr:") >> n: verify!(parse_number, |&n| n >= 1920 && n <= 2002) >> (Field::Byr(n as u32))));
    named!(iyr<&str, usize>, do_parse!(tag!("iyr:") >> n: parse_number >> (n)));
    named!(eyr<&str, usize>, do_parse!(tag!("eyr:") >> n: parse_number >> (n)));
    named!(hgt<&str, (usize, &str)>, do_parse!(tag!("hgt:") >> n: parse_number >> u: alt!(tag!("cm") | tag!("in")) >> (n, u)));
    named!(hcl<&str, &str>, do_parse!(tag!("hcl:") >> tag!("#") >> n: take!(6) >> (n)));
    named!(ecl<&str, &str>, do_parse!(tag!("ecl:") >> c: take!(3) >> (c)));
    named!(pid<&str, Vec<char>>, do_parse!(tag!("pid:") >> n: many_m_n!(9,9,satisfy(|c| c.is_ascii_digit())) >> (n)));
    named!(cid<&str, ()>, do_parse!(tag!("cid:") >> take_while1!(|c: char| !c.is_whitespace()) >> ()));

    fn parse_field2(input: &str) -> IResult<&str, Field> {
        Ok((input, Field::Cid))
    }

    pub fn parse_field<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
        let (input, field) = take(3usize)(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _content) = take_while1(|c: char| !c.is_whitespace())(input)?;
        Ok((input, field))
    }
    pub fn parse_passport<'a>(input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
        let (input, lo) = separated_list1(satisfy(|c| c.is_whitespace()), parse_field)(input)?;
        Ok((input, lo))
    }

    pub fn parse_passports<'a>(input: &'a str) -> IResult<&'a str, Vec<Vec<&'a str>>> {
        let (input, lo) = separated_list1(tag("\n\n"), parse_passport)(input)?;
        Ok((input, lo))
    }

    #[test]
    fn test_byr() {
        let x = byr("byr:1984").unwrap();
        assert_eq!(x, ("", Field::Byr(1984)));
    }
}
