use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
struct Assignment {
    lower: u32,
    upper: u32,
}

#[derive(Debug)]
struct AssignmentPair {
    first: Assignment,
    second: Assignment,
}

fn main() -> io::Result<()> {
    let lines = read_input("./examples/04/input.txt")?;
    let assignments = parse_input(&lines);

    println!("day 4.1 = {}", solve_part1(&assignments));
    println!("day 4.2 = {}", solve_part2(&assignments));

    Ok(())
}

fn solve_part1(assignments: &[AssignmentPair]) -> u32 {
    let mut num_fully_contained = 0;

    for AssignmentPair { first, second } in assignments {
        let first_in_second = first.lower >= second.lower && first.upper <= second.upper;
        let second_in_first = second.lower >= first.lower && second.upper <= first.upper;
        if first_in_second || second_in_first {
            num_fully_contained += 1;
        }
    }

    num_fully_contained
}

fn solve_part2(assignments: &[AssignmentPair]) -> u32 {
    let mut num_overlapping = 0;

    for AssignmentPair { first, second } in assignments {
        let is_overlapping = false
            || (first.lower..=first.upper).contains(&second.lower)
            || (first.lower..=first.upper).contains(&second.upper)
            || (second.lower..=second.upper).contains(&first.lower)
            || (second.lower..=second.upper).contains(&first.upper);

        if is_overlapping {
            num_overlapping += 1;
        }
    }

    num_overlapping
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

fn parse_input(lines: &[String]) -> Vec<AssignmentPair> {
    let mut assignments = Vec::new();
    for line in lines {
        assignments.push(parse_line(line));
    }
    assignments
}

fn parse_line(line: &str) -> AssignmentPair {
    let char_iter = &mut line.chars();
    let first = {
        let lower = char_iter
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let upper = char_iter
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        Assignment { lower, upper }
    };
    let second = {
        let lower = char_iter
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let upper = char_iter
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        Assignment { lower, upper }
    };

    AssignmentPair { first, second }
}
