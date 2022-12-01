use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let lines = read_input("./examples/01/input.txt")?;

    println!("day 1.1 = {:?}", solve_part1(&lines));
    println!("day 1.2 = {:?}", solve_part2(&lines));

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

fn solve_part1(lines: &[String]) -> i32 {
    let mut sums = Vec::<i32>::new();
    let mut lines_iter = lines.into_iter().peekable();
    while lines_iter.peek().is_some() {
        let sum = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.parse::<i32>().expect("string must a number"))
            .sum::<i32>();

        sums.push(sum);
    }

    *sums.iter().max().unwrap()
}

fn solve_part2(lines: &[String]) -> i32 {
    let mut sums = Vec::<i32>::new();
    let mut lines_iter = lines.into_iter().peekable();
    while lines_iter.peek().is_some() {
        let sum = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.parse::<i32>().expect("string must a number"))
            .sum::<i32>();

        sums.push(sum);
    }

    sums.sort_by(|a, b| b.cmp(a));

    sums[0] + sums[1] + sums[2]
}
