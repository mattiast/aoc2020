mod day01;
mod day02;
mod day03;

fn main() -> std::io::Result<()> {
    let x = day03::part2()?;
    println!("Hello, world! {}", x);
    Ok(())
}
