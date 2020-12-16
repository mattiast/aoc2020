mod day01;
mod day02;
mod day03;
mod day04;
mod day07;
mod day08;
mod day09;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    Day1 {
        #[structopt(long)]
        toka: bool,
    },
    Day2 {
        #[structopt(long)]
        toka: bool,
    },
    Day3 {
        #[structopt(long)]
        toka: bool,
    },
    Day4 {
        #[structopt(long)]
        toka: bool,
    },
    Day7 {
        #[structopt(long)]
        toka: bool,
    },
    Day8 {
        #[structopt(long)]
        toka: bool,
    },
    Day9 {
        #[structopt(long)]
        toka: bool,
    },
    Day11 {
        #[structopt(long)]
        toka: bool,
    },
    Day12 {
        #[structopt(long)]
        toka: bool,
    },
    Day13 {
        #[structopt(long)]
        toka: bool,
    },
    Day14 {
        #[structopt(long)]
        toka: bool,
    },
    Day15 {
        #[structopt(long)]
        n: usize,
    },
    Day16 {
        #[structopt(long)]
        toka: bool,
    },
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let x: i64 = match opt {
        Opt::Day1 { toka: false } => day01::part1()? as i64,
        Opt::Day1 { toka: true } => day01::part2()? as i64,
        Opt::Day2 { toka: false } => day02::part1()? as i64,
        Opt::Day2 { toka: true } => day02::part2()? as i64,
        Opt::Day3 { toka: false } => day03::part1()?,
        Opt::Day3 { toka: true } => day03::part2()?,
        Opt::Day4 { toka: false } => day04::part1()? as i64,
        Opt::Day4 { toka: true } => day04::part2()? as i64,
        Opt::Day7 { toka: false } => day07::part1()? as i64,
        Opt::Day7 { toka: true } => day07::part2()? as i64,
        Opt::Day8 { toka: false } => day08::part1()? as i64,
        Opt::Day8 { toka: true } => day08::part2()? as i64,
        Opt::Day9 { toka: false } => day09::part1()?,
        Opt::Day9 { toka: true } => day09::part2()?,
        Opt::Day11 { toka: false } => day11::part1()?,
        Opt::Day11 { toka: true } => day11::part2()?,
        Opt::Day12 { toka: false } => day12::part1()?,
        Opt::Day12 { toka: true } => day12::part2()?,
        Opt::Day13 { toka: false } => day13::part1()?,
        Opt::Day13 { toka: true } => day13::part2()?,
        Opt::Day14 { toka: false } => day14::part1()?,
        Opt::Day14 { toka: true } => day14::part2()?,
        Opt::Day15 { n } => day15::part1(n)?,
        Opt::Day16 { toka: false } => day16::part1()?,
        Opt::Day16 { toka: true } => day16::part2()?,
    };
    println!("{}", x);
    Ok(())
}
