use std::fs;

use day_20::{parse_input, enhance_image};

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use ndarray::{concatenate, stack, Array2, Axis};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("min risk: {}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    let (_, (enhancements, mut image)) = parse_input(input).unwrap();

    enhance_image(image, &enhancements, 2)
    
}

// fn enhance_image(enhancements: &Vec<char>, image: Array2<char>) -> Array2<char> {
//     let mut enhanced_image = Array2::from_elem((image.nrows(), image.ncols()), '0');
//     for ((this_row,this_col), _) in image.indexed_iter() {
//         let up = this_row.wrapping_sub(1);
//         let down = this_row + 1;
//         let left = this_col.wrapping_sub(1);
//         let right = this_col + 1;
//         let mut binary_string = String::with_capacity(9);
//        for (row, col) in [(up,left), (up,this_col), (up,right), (this_row,left), (this_row,this_col), (this_row,right), (down,left), (down,this_col), (down,right)] {
//            let val = image.get((row,col)).unwrap_or(&'0');
//            binary_string.push(*val);
//        }
//        let binary_number = usize::from_str_radix(&binary_string, 2).unwrap();
//        let new_val = enhancements.get(binary_number).unwrap();
//        enhanced_image[(this_row, this_col)] = *new_val;
//     }

//     enhanced_image
// }

fn pad_image(image: Array2<char>, pad_char: char) -> Array2<char> {
    let col_padding = Array2::from_elem((image.nrows(), 2), pad_char);
    let image = concatenate![Axis(1), col_padding, image, col_padding];
    let row_padding = Array2::from_elem((2, image.ncols()), pad_char);
    let image: Array2<char> = concatenate!(Axis(0), row_padding, image, col_padding);
    image
}

fn get_slice() {

}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&INPUT), 35);
    }
}
