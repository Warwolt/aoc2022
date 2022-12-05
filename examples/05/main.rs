use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
struct RearrengementProcedure {
    stacks: Vec<VecDeque<char>>,
    instructions: Vec<MoveInstruction>,
}

#[derive(Debug, Default)]
struct MoveInstruction {
    num_to_move: usize,
    from: usize,
    to: usize,
}

fn main() -> io::Result<()> {
    let lines = read_input("./examples/05/input.txt")?;

    println!("day 5.1 = {}", solve_part1(&lines));
    println!("day 5.2 = {}", solve_part2(&lines));

    Ok(())
}

fn solve_part1(lines: &[String]) -> String {
    let mut procedure = parse_input(&lines);

    for instruction in procedure.instructions {
        let from = instruction.from;
        let to = instruction.to;
        for _ in 0..instruction.num_to_move {
            let item = procedure.stacks[from].pop_back().unwrap();
            procedure.stacks[to].push_back(item);
        }
    }

    let answer = procedure
        .stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>();

    answer
}

fn solve_part2(lines: &[String]) -> String {
    let mut procedure = parse_input(&lines);

    for instruction in procedure.instructions {
        let from = instruction.from;
        let to = instruction.to;
        let mut items = VecDeque::<char>::new();
        for _ in 0..instruction.num_to_move {
            let item = procedure.stacks[from].pop_back().unwrap();
            items.push_front(item);
        }
        for item in items {
            procedure.stacks[to].push_back(item);
        }
    }

    let answer = procedure
        .stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>();

    answer
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

fn parse_input(input: &[String]) -> RearrengementProcedure {
    let mut procedure = RearrengementProcedure {
        stacks: Vec::new(),
        instructions: Vec::new(),
    };

    // parse initial stack arrangements
    let mut lines = input.iter();
    let mut stack_inputs = Vec::<Vec<Option<char>>>::new();
    loop {
        let line = lines.next().expect("Missing stack arrangement input");
        if line.is_empty() {
            break;
        }
        // loop through the columns
        let mut stack_input = Vec::<Option<char>>::new();
        for column in &line.chars().chunks(4) {
            let second_char = column.skip(1).next().unwrap();
            if second_char.is_digit(10) {
                break;
            }

            if second_char.is_whitespace() {
                stack_input.push(None);
            } else {
                stack_input.push(Some(second_char));
            }
        }

        stack_inputs.push(stack_input);
    }

    // add stack arrangements to procedure
    for _ in 0..stack_inputs.len() {
        procedure.stacks.push(VecDeque::new());
    }
    for stack_input in &stack_inputs {
        for (i, elem) in stack_input.iter().enumerate() {
            if let Some(elem) = elem {
                procedure.stacks[i].push_front(*elem);
            }
        }
    }

    // parse rearrengement instructions
    for line in lines {
        let mut instruction = MoveInstruction::default();
        let chars = &mut line.chars();
        instruction.num_to_move = chars
            .by_ref()
            .skip("move ".len())
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        instruction.from = chars
            .by_ref()
            .skip("from ".len())
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
            - 1;
        instruction.to = chars
            .by_ref()
            .skip("to ".len())
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
            - 1;

        procedure.instructions.push(instruction);
    }

    procedure
}
