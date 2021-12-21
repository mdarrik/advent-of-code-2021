use std::fs;

use day_20::{parse_input, enhance_image};

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("number of illuminated pixels: {}", part_2(&input));
}

fn part_2(input: &str) -> u32 {
    let (_, (enhancements, image)) = parse_input(input).unwrap();

    enhance_image(image, &enhancements, 50)
    
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");

    
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&INPUT), 3351);
    }
}
