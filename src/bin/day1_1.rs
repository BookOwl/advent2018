use std::io::Read;
use std::fs::{File};

fn main() -> std::io::Result<()> {
    // Get input from file
    let mut f = File::open("inputs/day1_1.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    
    // Sum the lines of input to get the final answer
    let freq = input.split("\n").map(|n| n.parse::<i32>().unwrap()).fold(0, |freq, change| freq + change);
    println!("{}", freq);

    Ok(())
}