mod day01;
mod day02;
mod day03;
mod day04;
mod day07;
mod day08;
mod day09;
mod day12;

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
    Day12 {
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
        Opt::Day7 { toka: true } => day07::part1()? as i64,
        Opt::Day8 { toka: false } => day08::part1()? as i64,
        Opt::Day8 { toka: true } => day08::part2()? as i64,
        Opt::Day9 { toka: false } => day09::part1()?,
        Opt::Day9 { toka: true } => day09::part2()?,
        Opt::Day12 { toka: false } => day12::part1()?,
        Opt::Day12 { toka: true } => day12::part2()?,
    };
    println!("{}", x);
    Ok(())
}
