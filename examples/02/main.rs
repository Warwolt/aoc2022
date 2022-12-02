use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Copy, Clone)]
struct Game {
    their_hand: Hand,
    our_hand: Hand,
}

#[derive(Debug, Copy, Clone)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

fn main() -> io::Result<()> {
    let lines = read_input("./examples/02/input.txt")?;

    println!("day 2.1 = {:?}", solve_part1(&lines));

    Ok(())
}

fn solve_part1(lines: &Vec<String>) -> u32 {
    let games = parse_input(&lines).expect("couldn't parse input");
    let total_score = games.iter().map(|game| score_game(*game)).sum::<u32>();
    total_score
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

fn parse_input(lines: &Vec<String>) -> Option<Vec<Game>> {
    let mut games = Vec::new();
    for line in lines {
        games.push(parse_line(line)?);
    }
    Some(games)
}

fn parse_line(line: &str) -> Option<Game> {
    let mut char_iter = line.chars();
    let their_hand = match char_iter.next()? {
        'A' => Some(Hand::Rock),
        'B' => Some(Hand::Paper),
        'C' => Some(Hand::Scissor),
        _ => None,
    }?;
    let our_hand = match char_iter.skip(1).next()? {
        'X' => Some(Hand::Rock),
        'Y' => Some(Hand::Paper),
        'Z' => Some(Hand::Scissor),
        _ => None,
    }?;

    Some(Game {
        their_hand,
        our_hand,
    })
}

fn score_game(game: Game) -> u32 {
    let shape_score = match game.our_hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissor => 3,
    };
    let round_score = match game_result(game) {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Loss => 0,
    };
    shape_score + round_score
}

fn game_result(game: Game) -> GameResult {
    match game.our_hand {
        Hand::Rock => match game.their_hand {
            Hand::Rock => GameResult::Draw,
            Hand::Paper => GameResult::Loss,
            Hand::Scissor => GameResult::Win,
        },
        Hand::Paper => match game.their_hand {
            Hand::Rock => GameResult::Win,
            Hand::Paper => GameResult::Draw,
            Hand::Scissor => GameResult::Loss,
        },
        Hand::Scissor => match game.their_hand {
            Hand::Rock => GameResult::Loss,
            Hand::Paper => GameResult::Win,
            Hand::Scissor => GameResult::Draw,
        },
    }
}
