use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn read_input() -> io::Result<(i128, Vec<Option<i128>>)> {
    let file = File::open("data/day13.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let line = lines.next().unwrap()?;
    let x: i128 = line.parse().unwrap();

    let line = lines.next().unwrap()?;
    let v: Vec<Option<i128>> = line
        .split(',')
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect();

    Ok((x, v))
}

pub fn part1() -> io::Result<i64> {
    let (x, v) = read_input()?;

    let i = v
        .into_iter()
        .filter_map(|i| i)
        .min_by_key(|i| i - x % i)
        .unwrap();

    let ret = i * (i - x % i);
    Ok(ret as i64)
}
pub fn part2() -> io::Result<i64> {
    let (_, v) = read_input()?;

    let rem_n_mod: Vec<_> = v
        .into_iter()
        .enumerate()
        .filter_map(|(i, mbus)| mbus.map(|bus| (-(i as i128), bus)))
        .collect();

    let (mut x, m) = chinese_remainder(&rem_n_mod).unwrap();

    if x < 0 {
        x += m;
    }

    Ok(x as i64)
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(res_n_mod: &[(i128, i128)]) -> Option<(i128, i128)> {
    let prod = res_n_mod.iter().map(|(_, m)| m).product::<i128>();

    let mut sum = 0;

    for &(residue, modulus) in res_n_mod.iter() {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p;
        sum %= prod;
    }

    Some((sum, prod))
}

#[test]
fn test_chinese_remainder() {
    let rem_n_mods = vec![(1, 31), (2, 57), (3, 71), (4, 97)];
    let (x, _) = chinese_remainder(&rem_n_mods).unwrap();

    for (r, m) in rem_n_mods {
        assert_eq!(r, x % m);
    }
}
