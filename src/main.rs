mod day01;
mod day02;
mod day03;
mod day04;

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
    Day4,
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
        Opt::Day4 => day04::part1()? as i64,
    };
    println!("{}", x);
    Ok(())
}
