#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

use serde::Deserialize;
use std::fs;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./q2.input.txt").unwrap();
    dbg!(input.clone());
    let p1_output = p1(input.clone());
    // let instructions = q1_input.input.clone();
    // let p1_output = p1(instructions.clone());
    println!("p1: {}", p1_output);
    // let p2_output = p2(instructions.clone());
    // println!("p2: {}", p2_output);
}

fn p1(instructions: String) -> i32 {
    let final_position =
        instructions
            .lines()
            .fold(Position::default(), |mut position, command_line| {
                let command_split: Vec<&str> = command_line.split(' ').collect();
                let distance = command_split[1].parse::<i32>().unwrap();
                match command_split[0] {
                    "forward" => position.horizontal += distance,
                    "down" => position.depth += distance,
                    "up" => position.depth -= distance,
                    command => panic!("invalid direction {}", command),
                };
                position
            });

    // let parsed_commands = parse_commands(instructions);
    // let final_position =
    //     parsed_commands
    //         .into_iter()
    //         .fold(Position::default(), |mut position, command| {
    //             match command.direction {
    //                 DirectionOld::Forward => position.horizontal += command.distance,
    //                 DirectionOld::Down => position.depth += command.distance,
    //                 DirectionOld::Up => {
    //                     if position.depth > command.distance {
    //                         position.depth -= command.distance;
    //                     } else {
    //                         position.depth = 0;
    //                     }
    //                 }
    //             };
    //             position
    //         });
    final_position.depth * final_position.horizontal
}

fn p2(instructions: Vec<String>) -> i32 {
    let parsed_commands = parse_commands(instructions);

    let final_position =
        parsed_commands
            .into_iter()
            .fold(Position::default(), |mut position, command| {
                match command.direction {
                    DirectionOld::Forward => {
                        position.horizontal += command.distance;
                        position.depth += command.distance * position.aim;
                    }
                    DirectionOld::Up => {
                        position.aim -= command.distance;
                    }
                    DirectionOld::Down => {
                        position.aim += command.distance;
                    }
                };
                position
            });

    final_position.depth * final_position.horizontal
}

fn parse_commands(instructions: Vec<String>) -> Vec<Command> {
    instructions
        .into_iter()
        .map(|command_string| {
            let pairs: Vec<&str> = command_string.split(" ").into_iter().collect();
            let direction_string = pairs[0];
            let distance = pairs[1].parse::<i32>().unwrap();

            let direction = match direction_string {
                "forward" => DirectionOld::Forward,
                "down" => DirectionOld::Down,
                "up" => DirectionOld::Up,
                _ => DirectionOld::Up,
            };
            Command {
                direction,
                distance,
            }
        })
        .collect::<Vec<Command>>()
}

#[derive(Debug, Deserialize)]
struct QuestionInput {
    input: Vec<String>,
}

#[derive(Default)]
struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

struct Command {
    direction: DirectionOld,
    distance: i32,
}

enum DirectionOld {
    Forward,
    Down,
    Up,
}

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[test]
fn part_1_example() {
    let commands = "forward 5 
down 5
forward 8
up 3
down 8
forward 2".to_string();
    dbg!(commands.clone());
    assert_eq!(p1(commands), 150);
}

#[test]
fn part_2_example() {
    let commands = vec![
        "forward 5".to_string(),
        "down 5".to_string(),
        "forward 8".to_string(),
        "up 3".to_string(),
        "down 8".to_string(),
        "forward 2".to_string(),
    ];

    assert_eq!(p2(commands), 900);
}
