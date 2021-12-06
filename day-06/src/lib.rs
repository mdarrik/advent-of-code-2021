use nom::{bytes::complete::tag, character::complete::u8, multi::separated_list1, IResult};

use std::collections::VecDeque;

pub fn model_fish(input: &str, days: u16) -> u64 {
    let (_, initial_state) = input_list(input).unwrap();
    let mut fish_population: VecDeque<u64> = VecDeque::from([0; 9]);
    initial_state.iter().for_each(|day| {
        fish_population[*day as usize] += 1;
    });
    for _ in 0..days as u16 {
        let reproducing_fish = fish_population.pop_front().unwrap();
        fish_population.push_back(reproducing_fish);
        fish_population[6] += reproducing_fish;
    }
    fish_population.iter().sum()
}
fn input_list(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), u8)(input)
}
