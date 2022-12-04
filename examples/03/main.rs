use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let lines = read_input("./examples/03/input.txt")?;

    println!("day 3.1 = {}", solve_part1(&lines));
    println!("day 3.2 = {}", solve_part2(&lines));

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

fn solve_part1(lines: &[String]) -> u32 {
    let mut duplicate_items = Vec::new();

    for line in lines {
        let len = line.len();
        let (front, back) = (&line[0..len / 2], &line[len / 2..len]);
        for item in front.chars() {
            if back.contains(item) {
                duplicate_items.push(item.clone());
                break;
            }
        }
    }

    let sum = duplicate_items
        .iter()
        .map(|i| item_priority(*i))
        .sum::<u32>();

    sum
}

fn solve_part2(lines: &[String]) -> u32 {
    let mut badge_items = Vec::new();

    for i in (0..lines.len()).step_by(3) {
        let group = (&lines[i], &lines[i + 1], &lines[i + 2]);
        for item in group.0.chars() {
            if group.1.contains(item) && group.2.contains(item) {
                badge_items.push(item);
                break;
            }
        }
    }

    let sum = badge_items.iter().map(|i| item_priority(*i)).sum::<u32>();

    sum
}

fn item_priority(item: char) -> u32 {
    if !item.is_ascii_alphabetic() {
        return 0;
    }

    if item.is_lowercase() {
        1 + item as u32 - ('a' as u32)
    } else {
        27 + item as u32 - ('A' as u32)
    }
}
