use color_eyre::Report;
use std::{fs, time::Instant};

// Importing modules
mod analyze;

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    // Read the content of the file
    let input = fs::read_to_string("dutch.txt")?;
    //let word = "universityoftwente";
    let word = "universityoftwente";

    let start = Instant::now();
    analyze::analyze(&input, &word);

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    Ok(())
}
