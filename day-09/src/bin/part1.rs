use std::{cmp::min, fs};

use day_09::elevation_map;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use ndarray::{s, Array2};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("danger rating: {}", find_local_min(&input));
}

fn find_local_min(input: &str) -> u32 {
    let (_, map_as_vec) = elevation_map(input).unwrap();
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();
    let map_of_elevations = Array2::from_shape_vec((num_rows, num_cols), map_as_vec).unwrap();
    // let mut min_sum = map_of_elevations.windows((3,3)).into_iter().fold(0u32, |mut sum, window| {
    //     let middle = window.get((1,1)).unwrap();
    //     if middle == window.iter().min().unwrap() {
    //         sum += middle.clone() as u32;
    //     }
    //     sum
    // });
    let mut sum = 0u32;
    for row in 0..num_rows {
        for col in 0..num_cols {
            let value = map_of_elevations.get((row, col)).unwrap();
            let sl = s![
                (row.checked_sub(1).unwrap_or_default())..=min(row + 1, num_rows - 1),
                (col.checked_sub(1).unwrap_or_default())..=min(col + 1, num_cols - 1)
            ];
            let local_slice = map_of_elevations.slice(sl);
            if local_slice.iter().min().unwrap() == value {
                sum += (*value as u32) + 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part_1() {
        assert_eq!(find_local_min(&INPUT), 15);
    }
}
