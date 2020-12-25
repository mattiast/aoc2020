fn step(cups: [u8; 9]) -> [u8; 9] {
    // i = how many steps from current to destination
    let i = cups[4..9]
        .iter()
        .enumerate()
        .map(|(i, c)| ((9 + cups[0] - c) % 9, i))
        .min()
        .unwrap()
        .1
        + 1;
    let mut result = cups.clone();
    result[1..1 + i].clone_from_slice(&cups[4..4 + i]);
    result[1 + i..4 + i].clone_from_slice(&cups[1..4]);
    rotate(result)
}

fn rotate(cups: [u8; 9]) -> [u8; 9] {
    let mut result = [0; 9];
    result[0..8].clone_from_slice(&cups[1..9]);
    result[8] = cups[0];
    result
}
use std::io;
pub fn part1() -> io::Result<i64> {
    let mut cups: [u8; 9] = [5, 3, 8, 9, 1, 4, 7, 6, 2];
    //let mut cups: [u8; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    for _ in 0..100 {
        cups = step(cups);
    }
    println!("{:?}", cups);
    Ok(4)
}
pub fn part2() -> io::Result<i64> {
    todo!()
}
