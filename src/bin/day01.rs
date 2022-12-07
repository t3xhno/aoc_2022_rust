use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let data = fs::read_to_string("data/day01_example.txt")?;

    let mut parsed_data: Vec<u32> = data
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    // I WANT TO SORT IN PLACE!!!
    parsed_data.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", parsed_data[0]);
    println!("Part 2: {:?}", parsed_data.iter().take(3).sum::<u32>());

    Ok(())
}
