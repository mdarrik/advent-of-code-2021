use std::{cmp::min, fs};

use day_11::{parse_input, Octopus};
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
    println!("number of flashes: {}", model_octopus(&input, 100));
}

fn model_octopus(input: &str, days: u16) -> u32 {
    let (_, vals) = parse_input(input).unwrap();
    let mut val_matrix = Array2::from_shape_vec((10, 10), vals).unwrap();
    let mut flash_count = 0u32;
    for _ in 0..days {
        val_matrix.map_inplace(|v| {
            v.increment_flash();
        });

        loop {
            let mut has_not_flashed = true;
            for (row, col) in val_matrix
                .indexed_iter()
                .filter_map(|(position, octopus)| {
                    if octopus.needs_to_flash() {
                        Some(position)
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
            {
                if has_not_flashed {
                    has_not_flashed = false;
                }

                val_matrix.get_mut((row, col)).unwrap().expend_flash();
                flash_count += 1;
                let min_row = row.saturating_sub(1);
                let max_row = min(row + 1, val_matrix.nrows() - 1);
                let min_col = col.saturating_sub(1);
                let max_col = min(col + 1, val_matrix.ncols() - 1);

                let mut octo_slice = val_matrix.slice_mut(s![min_row..=max_row, min_col..=max_col]);
                octo_slice.map_inplace(|octopus| octopus.increment_flash());
            }
            if has_not_flashed {
                break;
            }
        }
        val_matrix.map_inplace(|octopus| {
            if *octopus == Octopus::Flashed {
                *octopus = Octopus::Power(0)
            }
        });
    }

    flash_count
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part_1() {
        assert_eq!(model_octopus(&INPUT, 100), 1656);
    }
    #[test]
    fn part_1_2_days() {
        assert_eq!(model_octopus(&INPUT, 2), 35);
    }
}
