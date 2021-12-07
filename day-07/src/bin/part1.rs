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

pub fn calc_shortest_distance(input: &str) -> u64 {
    let (_, positions) = crab_positions(input).unwrap();
    let max_position = positions.iter().max().unwrap();
    let mut weighted_positions: BTreeMap<u16, u16> = BTreeMap::default();

    positions.iter().for_each(|position| {
        weighted_positions
            .entry(*position)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    });

    let mut distance_map = vec![0u64; (max_position + 1) as usize];

    weighted_positions.iter().for_each(|(position, number)| {
        for final_position in 0..=*max_position {
            if position > &final_position {
                let added_weight = (position - final_position) as u64 * *number as u64;
                distance_map[final_position as usize] += added_weight;
            } else {
                let added_weight = (final_position - position) as u64 * *number as u64;
                distance_map[final_position as usize] += added_weight;
            }
        }
    });

    *distance_map.iter().min().unwrap()
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
