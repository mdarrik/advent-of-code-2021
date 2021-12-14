use std::{cmp::min, fs};

use day_13::input_instructions;
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
    println!("number of points after 1 fold: {}", fold_paper_part1(&input));
}

fn fold_paper_part1(input: &str) -> usize {
    let (_, (mut points, instructions)) = input_instructions(&input).unwrap();
    match *instructions.first().unwrap() {
        day_13::Instruction::X(line) => {
            let old_points = points.clone();
            let points_to_fold = old_points.iter().filter(|(x, _)| *x > line );
            points.retain(|(x,_)| *x < line);
            for (old_x, old_y) in points_to_fold {
                let new_x = line - (old_x - line);
                let new_point = (new_x, *old_y);
                points.insert(new_point);

            }
        },
        day_13::Instruction::Y(line) => {
            let old_points = points.clone();
            let points_to_fold = old_points.iter().filter(|(_,y)| *y > line);
            points.retain(|(_,y)| *y < line);
            for (old_x, old_y) in points_to_fold {
                let new_y = line - (old_y - line);
                let new_point = (*old_x, new_y);
                points.insert(new_point);

            }
        },
    };



    points.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part_1() {
        assert_eq!(fold_paper_part1(&INPUT), 17);
    }
}
