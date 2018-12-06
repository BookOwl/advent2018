use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn part1() -> std::io::Result<()> {
    // Get input from file
    let mut f = File::open("inputs/day1.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    
    // Sum the lines of input to get the final answer
    let freq: i32 = input.split("\n").map(|n| n.parse::<i32>().unwrap()).sum();
    println!("Part 1: {}", freq);

    Ok(())
}

fn part2() -> std::io::Result<()> {
    // Get input from file
    let mut f = File::open("inputs/day1.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    
    // Get the changes in frequency
    let freq_changes = input.split("\n").map(|n| n.parse::<i32>().unwrap()).cycle();
    let mut freq = 0;

    // Create a HashSet to store all the freqs we generate.
    // We can't just use a Vec because the O(n) performance of Vec.conatains()
    // dominates the run time and makes it take forever.
    let mut freqs: HashSet<i32> = HashSet::new();

    for change in freq_changes {
        freq += change;
        if freqs.contains(&freq) {
            break
        } else {
            freqs.insert(freq);
        }
    }
    println!("Part 2: {}", freq);

    Ok(())
}

fn main() -> std::io::Result<()> {
    part1()?;
    part2()?;
    Ok(())
}