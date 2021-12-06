use std::fs;

use day_06::model_fish;

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("Fish after 80 days: {}", model_fish(&input, 80));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn after_80_days() {
        assert_eq!(model_fish(&INPUT, 80), 5934);
    }
    #[test]
    fn after_18_days() {
        assert_eq!(model_fish(&INPUT, 18), 26);
    }
}
