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

pub fn part2() -> io::Result<usize> {
    let input = std::fs::read_to_string("data/day04.txt")?;
    let (_, pss) = parsing::parse_passports2(&input).unwrap();
    for ps in pss {
        println!("{:?}", ps);
    }
    Ok(4)
}
#[derive(Debug, PartialEq)]
pub enum HUnit {
    In,
    Cm,
}
#[derive(Debug, PartialEq)]
pub enum Ecl {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

#[derive(Debug, PartialEq)]
pub enum Field {
    Byr(u32),
    Iyr(u32),
    Eyr(u32),
    Hgt(u32, HUnit),
    Hcl(u8, u8, u8),
    Ecl(Ecl),
    Pid(u64),
    Cid,
}

mod parsing {
    use super::{Ecl, Field, HUnit};
    use nom::{
        alt,
        bytes::complete::{tag, take, take_while1},
        character::complete::satisfy,
        combinator::verify,
        do_parse,
        multi::separated_list1,
        named, tag, take, take_while1, verify, IResult,
    };
    use nom::{combinator::map_res, take_while_m_n};
    fn my_err() -> nom::Err<nom::error::Error<&'static str>> {
        nom::Err::Failure(nom::error::Error::new("foo", nom::error::ErrorKind::Alpha))
    }
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
    named!(iyr<&str, Field>, do_parse!(tag!("iyr:") >> n: verify!(parse_number, |&n| n >= 2010 && n <= 2020) >> (Field::Iyr(n as u32))));
    named!(eyr<&str, Field>, do_parse!(tag!("eyr:") >> n: verify!(parse_number, |&n| n >= 2020 && n <= 2030) >> (Field::Eyr(n as u32))));
    named!(hgt1<&str, (usize, &str)>, do_parse!(tag!("hgt:") >> n: parse_number >> u: alt!(tag!("cm") | tag!("in")) >> (n, u)));
    named!(hcl1<&str, &str>, do_parse!(tag!("hcl:") >> tag!("#") >> n: take_while_m_n!(6,6,|c:char| c.is_ascii_hexdigit()) >> (n)));
    named!(ecl1<&str, &str>, do_parse!(tag!("ecl:") >> c: take!(3) >> (c)));
    named!(pid<&str, Field>, do_parse!(tag!("pid:") >> n: parse_number >> (Field::Pid(n as u64))));
    named!(cid<&str, Field>, do_parse!(tag!("cid:") >> take_while1!(|c: char| !c.is_whitespace()) >> (Field::Cid)));

    fn hgt(input: &str) -> IResult<&str, Field> {
        let (input, (x, u)) = verify(hgt1, |(x, u)| {
            (u == &"cm" && *x >= 150 && *x <= 193) || (u == &"in" && *x >= 59 && *x <= 76)
        })(input)?;

        match u {
            "in" => Ok((input, Field::Hgt(x as u32, HUnit::In))),
            "cm" => Ok((input, Field::Hgt(x as u32, HUnit::Cm))),
            _ => panic!("yo"),
        }
    }
    fn hcl(input: &str) -> IResult<&str, Field> {
        let (input, x) = hcl1(input)?;
        let a = u8::from_str_radix(&x[0..2], 16).map_err(|_| my_err())?;
        let b = u8::from_str_radix(&x[2..4], 16).map_err(|_| my_err())?;
        let c = u8::from_str_radix(&x[4..6], 16).map_err(|_| my_err())?;
        Ok((input, Field::Hcl(a, b, c)))
    }
    fn ecl(input: &str) -> IResult<&str, Field> {
        let (input, x) = ecl1(input)?;
        let c = match x {
            "amb" => Ok(Ecl::Amb),
            "blu" => Ok(Ecl::Blu),
            "brn" => Ok(Ecl::Brn),
            "gry" => Ok(Ecl::Gry),
            "grn" => Ok(Ecl::Grn),
            "hzl" => Ok(Ecl::Hzl),
            "oth" => Ok(Ecl::Oth),
            _ => Err(my_err()),
        }?;
        Ok((input, Field::Ecl(c)))
    }

    named!(parse_field2<&str, Field>, alt!(byr | iyr | eyr | pid | cid | hgt | hcl | ecl));

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
    pub fn parse_passport2<'a>(input: &'a str) -> IResult<&'a str, Vec<Field>> {
        let (input, lo) = separated_list1(satisfy(|c| c.is_whitespace()), parse_field2)(input)?;
        Ok((input, lo))
    }

    pub fn parse_passports2<'a>(input: &'a str) -> IResult<&'a str, Vec<Vec<Field>>> {
        let (input, lo) = separated_list1(tag("\n\n"), parse_passport2)(input)?;
        Ok((input, lo))
    }

    #[test]
    fn test_byr() {
        let x = byr("byr:1984").unwrap();
        assert_eq!(x, ("", Field::Byr(1984)));
    }
    #[test]
    fn test_ecl() {
        let x = hcl("hcl:#010203").unwrap();
        assert_eq!(x, ("", Field::Hcl(1, 2, 3)));
    }
}
