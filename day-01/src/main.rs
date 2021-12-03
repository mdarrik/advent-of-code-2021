use serde::Deserialize;
use std::error::Error;
use std::fs;

fn q1_p1(depths: Vec<i32>) -> i32 {
    depths.windows(2).fold(0, |number_of_increases, window| {
        if window[0] < window[1] {
            return number_of_increases + 1;
        }
        return number_of_increases;
    })
}

fn q1_p2(depths: Vec<i32>) -> i32 {
    let triplets: Vec<i32> = depths
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect();
    triplets.windows(2).fold(0, |number_of_increases, window| {
        if window[0] < window[1] {
            number_of_increases + 1
        } else {
            number_of_increases
        }
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("q1.input.json")?;
    let q1_input: Q1Input = serde_json::from_str(&file)?;

    let depths = q1_input.input.clone();
    let increasing_depths = q1_p1(depths.clone());
    println!("p1: {}", increasing_depths);
    let increasing_sums = q1_p2(depths.clone());
    println!("p2: {}", increasing_sums);
    return Ok(());
}

#[derive(Debug, Deserialize)]
struct Q1Input {
    input: Vec<i32>,
}

#[test]
fn part_1_example() {
    let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec();
    assert_eq!(q1_p1(depths), 7);
}
#[test]
fn part_2_example() {
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(q1_p2(depths), 5);
}
