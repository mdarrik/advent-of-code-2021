use std::fs;

use day_14::build_polymer;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!(
        "score of polymer after 10 steps: {}",
        build_polymer(&input, 10)
    );
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(build_polymer(&INPUT, 10), 1588);
    }
}
