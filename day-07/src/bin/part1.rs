use std::{collections::BTreeMap, fs};

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

use day_07::crab_positions;
fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    let fuel_cost = calc_shortest_distance(&input);
    println!("Minimum fuel cost is {}", fuel_cost);
}

pub fn calc_shortest_distance(input: &str) -> u32 {
    let (_, mut positions) = crab_positions(input).unwrap();
    // let max_position = positions.iter();
    // let mut weighted_positions: BTreeMap<u16, u16> = BTreeMap::default();
    positions.sort_unstable();

    let median = *positions.get((positions.len() + 1) / 2).unwrap();

    positions.iter().fold(0u32, |total, pos| {
        total + (*pos as i32 - median as i32).abs() as u32
    })

    // positions.iter().for_each(|position| {
    //     weighted_positions
    //         .entry(*position)
    //         .and_modify(|val| *val += 1)
    //         .or_insert(1);
    // });

    // // let mut distance_map = vec![0u32; (max_position + 1) as usize];

    // weighted_positions
    //     .iter()
    //     .map(|(position, number)| {
    //         let mut total_fuel: u32 = 0;
    //         for final_position in 0..=*max_position {
    //             if position > &final_position {
    //                total_fuel += (position - final_position) as u32 * *number as u32
    //             } else {
    //                total_fuel += (final_position - position) as u32 * *number as u32
    //             }
    //         }
    //         total_fuel
    //     })
    //     .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = include_str!("./test-input.txt");
    #[test]
    fn part_1() {
        assert_eq!(calc_shortest_distance(INPUT), 37);
    }
}
