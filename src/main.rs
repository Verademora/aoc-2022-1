use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // Split the input and collect
    let v: Vec<&str> = buffer.trim_end().split("\n\n").collect();

    // Recollect all sums into a new collection
    let mut sums: Vec<i32> = Vec::new();
    for elf in v {
        let v2: Vec<i32> = elf.split('\n')
            .filter_map(|s| s.parse().ok())
            .collect();
        sums.push(v2.iter().sum());
    }
    
    // Sort by largest and collect the top 3
    sums.sort_by(|a, b| b.cmp(a));
    let top_three_sum: i32 = sums.iter().take(3).sum();

    println!("The largest sum is: {}", sums[0]);
    println!("The largest 3 sums add up to: {}", top_three_sum);

    Ok(())
}
