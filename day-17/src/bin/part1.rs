use std::fs;

use day_17::parse_input;

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

fn part_1(input: &str) -> i32 {
    let (_, target_area) = parse_input(input).unwrap();

    let min_y = target_area.y.start();

    // the absolute max height you can reach and still be in range (because after crossing zero the negative velocity will be whatever you started with + 1)
    let absolute_max_y_velocity = min_y.abs() as i32 - 1;

    (absolute_max_y_velocity * (absolute_max_y_velocity + 1))/ 2
    
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");

    
    #[test]
    fn test_part_1_input_1() {
        assert_eq!(part_1(&INPUT), 45);
    }
}
