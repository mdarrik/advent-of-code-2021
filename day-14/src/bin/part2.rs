use std::fs;

use day_14::build_polymer_2;
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
        "score of polymer after 40 steps: {}",
        build_polymer_2(&input, 40)
    );
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn test_part_2() {
        assert_eq!(build_polymer_2(&INPUT, 40), 2188189693529);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(build_polymer_2(&INPUT, 10), 1588);
    }

    #[test]
    fn test_1_step() {
        assert_eq!(build_polymer_2(&INPUT, 1), 1);
    }
}
