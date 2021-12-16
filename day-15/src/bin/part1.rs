use std::fs;

use day_15::parse_input;

use day_15::shortest_distance;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("min risk: {}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    let (_, value_map) = parse_input(input).unwrap();
    let end_point = (value_map.nrows() as u16 - 1, value_map.ncols() as u16 - 1);
    shortest_distance((0, 0), &value_map, end_point)
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&INPUT), 40);
    }
}
