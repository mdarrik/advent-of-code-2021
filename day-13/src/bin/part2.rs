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
    fold_paper_part2(&input);
}

fn fold_paper_part2(input: &str) {
    let (_, (mut points, instructions)) = input_instructions(&input).unwrap();
    for instruction in instructions {
        match instruction {
            day_13::Instruction::X(line) => {
                let old_points = points.clone();
                let points_to_fold = old_points.iter().filter(|(x, _)| *x > line);
                points.retain(|(x, _)| *x < line);
                for (old_x, old_y) in points_to_fold {
                    let new_x = line - (old_x - line);
                    let new_point = (new_x, *old_y);
                    points.insert(new_point);
                }
            }
            day_13::Instruction::Y(line) => {
                let old_points = points.clone();
                let points_to_fold = old_points.iter().filter(|(_, y)| *y > line);
                points.retain(|(_, y)| *y < line);
                for (old_x, old_y) in points_to_fold {
                    let new_y = line - (old_y - line);
                    let new_point = (*old_x, new_y);
                    points.insert(new_point);
                }
            }
        };
    }
    let max_row:usize = points.clone().into_iter().max_by_key(|(_,y)|*y).unwrap().1.into();
    let max_col:usize = points.clone().into_iter().max_by_key(|(x,_)| *x).unwrap().0.into();
    let mut array = Array2::from_elem((max_row + 1, max_col + 1), " ");
    for (col, row) in points {
        array[(row as usize, col as usize)] = "#";
    }

    println!("{}", array);
    fs::write("./part-2.out.txt", format!("{}", array)).unwrap();

}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part_2() {
        fold_paper_part2(INPUT);
        assert_eq!(true, false);
    }
}
