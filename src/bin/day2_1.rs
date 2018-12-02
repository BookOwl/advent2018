use std::io::{BufRead, BufReader, Result};
use std::fs::File;


fn main() -> std::io::Result<()> {
    // Get input from file
    let f = File::open("inputs/day2.txt")?;
    let mut two_count = 0;
    let mut three_count = 0;
    
    for word in BufReader::new(f).lines() {
        let (has_two, has_three) = get_counts(&word?);
        two_count += has_two;
        three_count += has_three;
    }

    let checksum = two_count * three_count;
    println!("{}", checksum);

    Ok(())
}

fn get_counts(word: &str) -> (usize, usize) {
    let mut has_two = 0;
    let mut has_three = 0;
    for c in word.chars() {
        let count = word.matches(c).count();
        match count {
            2 => has_two = 1,
            3 => has_three = 1,
            _ => {},
        }
    };
    (has_two, has_three)
}