mod day01;
mod day02;

fn main() -> std::io::Result<()> {
    let x = day02::part1()?;
    println!("Hello, world! {}", x);
    Ok(())
}
