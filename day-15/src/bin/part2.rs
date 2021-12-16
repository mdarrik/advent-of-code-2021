use std::fs;

use day_15::parse_input;

use day_15::shortest_distance;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use ndarray::concatenate;
use ndarray::ArrayBase;
use ndarray::Axis;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("min risk: {}", part_2(&input));
}

fn part_2(input: &str) -> u32 {
    let (_, initial_map) = parse_input(input).unwrap();
    let first_row_arrs = (0..5_u8)
        .map(|row_tile_index| {
            initial_map.mapv(|v| {
                let new_val = v + row_tile_index;
                if new_val >= 10 {
                    (new_val % 10) + 1
                } else {
                    new_val
                }
            })
        })
        .collect::<Vec<_>>();
    let first_row_views = first_row_arrs.iter().map(|v| v.view()).collect::<Vec<_>>();
    let first_row = concatenate(Axis(0), &first_row_views[..]).unwrap();

    let col_vecs = (0..5u8)
        .map(|col_index| {
            first_row.mapv(|v| {
                let new_val = v + col_index;
                if new_val >= 10 {
                    (new_val % 10) + 1
                } else {
                    new_val
                }
            })
        })
        .collect::<Vec<ArrayBase<_, _>>>();
    let col_views = col_vecs.iter().map(|v| v.view()).collect::<Vec<_>>();
    let value_map = concatenate(Axis(1), &col_views[..]).unwrap();
    let end_point = (value_map.nrows() as u16 - 1, value_map.ncols() as u16 - 1);
    shortest_distance((0, 0), &value_map, end_point)
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&INPUT), 315);
    }
}
