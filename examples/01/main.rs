use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let lines = read_input("./examples/01/sample.txt")?;

    let mut lines_iter = lines.into_iter();
    let first_batch = lines_iter.by_ref().take_while(|line| !line.is_empty());

    let first_sum = first_batch
        .into_iter()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().expect("string must a number"))
        .sum::<i32>();

    println!("{}", first_sum);

    let _second_batch = lines_iter.by_ref().take_while(|line| !line.is_empty());

    Ok(())
}

fn read_input(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let file_content = BufReader::new(file);
    let lines = file_content
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();
    Ok(lines)
}
