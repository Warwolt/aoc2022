use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
struct RearrengementProcedure {
    stacks: Vec<Vec<char>>,
    _instructions: Vec<MoveInstruction>,
}

#[derive(Debug)]
struct MoveInstruction {
    _num: u32,
    _from: u32,
    _to: u32,
}

fn main() -> io::Result<()> {
    let lines = read_input("./examples/05/sample.txt")?;
    let procedure = parse_input(&lines);

    dbg!(&lines);
    dbg!(&procedure);

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

fn parse_input(input: &[String]) -> RearrengementProcedure {
    let mut procedure = RearrengementProcedure {
        stacks: Vec::new(),
        _instructions: Vec::new(),
    };

    // parse initial stack arrangements
    let mut lines = input.iter();
    let mut stack_inputs = Vec::<Vec<Option<char>>>::new();
    loop {
        let line = lines.next().expect("Missing stack arrangement input");
        println!("parsing: {}", line);
        if line.is_empty() {
            break;
        }
        // loop through the columns
        let mut stack_input = Vec::<Option<char>>::new();
        for column in &line.chars().chunks(4) {
            // (these next two lines only debugging)
            let column_str = column.collect::<String>();
            println!("\"{}\"", column_str);

            let second_char = column_str.chars().skip(1).next().unwrap();
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
    for _ in 0..stack_inputs.len() - 1 {
        procedure.stacks.push(Vec::new());
    }
    for stack_input in &stack_inputs {
        for (i, elem) in stack_input.iter().enumerate() {
            if let Some(elem) = elem {
                procedure.stacks[i].push(*elem);
            }
        }
    }

    println!("{:?}", &stack_inputs);

    // parse rearrengement instructions

    procedure
}
