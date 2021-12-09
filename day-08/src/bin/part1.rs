use std::fs;

use day_08::input_signals_part_1;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", get_digits_with_unique_string(&input));
}

fn get_digits_with_unique_string(input: &str) -> usize {
    let (_, display_digits) = input_signals_part_1(input).unwrap();
    let sizes_of_concern = [2u8, 4u8, 3u8, 7u8];
    dbg!(&display_digits);
    display_digits
        .iter()
        .filter(|display| sizes_of_concern.contains(&(display.len() as u8)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./test-input.txt");
    #[test]
    fn part_1() {
        assert_eq!(get_digits_with_unique_string(INPUT), 26);
    }
}
