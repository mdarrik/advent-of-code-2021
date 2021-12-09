use std::{collections::HashSet, fs};

use day_08::parse_inputs_part_2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", sum_outputs(&input));
}

fn sum_outputs(input: &str) -> u32 {
    let (_, mut lines) = parse_inputs_part_2(input).unwrap();
    lines.iter_mut().map(calc_line).sum()
}

fn calc_line((signals, digits): &mut (Vec<HashSet<char>>, Vec<HashSet<char>>)) -> u32 {
    // sort the vec by length
    signals.sort_unstable_by(|prev, next| prev.len().cmp(&next.len()));
    // one is the smallest, so it's at the first position.
    let one = signals.remove(0);
    // seven is the next smallest, so grab it
    let seven = signals.remove(0);
    // With 1, 7 removed, 4 will be the next smallest
    let four = signals.remove(0);
    // 8 is the last one since it has all the lines
    let eight = signals.pop().unwrap();
    // these are length 5
    let mut two: Option<&HashSet<char>> = None;
    let mut three: Option<&HashSet<char>> = None;
    let mut five: Option<&HashSet<char>> = None;
    // these are length 6
    let mut six: Option<&HashSet<char>> = None;
    let mut nine: Option<&HashSet<char>> = None;
    let mut zero: Option<&HashSet<char>> = None;
    for signal in signals {
        match signal.len() as u8 {
            5 => {
                // if the number - 1 only has 3 leftover, it has to be 3
                if signal.difference(&one).count() == 3 {
                    three = Some(signal);
                    // if the number + the numbers in 4 == 8 (which has len(7)) it must be 2
                } else if signal.union(&four).count() == 7 {
                    two = Some(signal);
                } else {
                    // if it's not either of those, it must be 2
                    five = Some(signal);
                }
            }
            6 => {
                // 6 union 1 will return 8, the others stay the same
                if signal.union(&one).count() == 7 {
                    six = Some(signal);
                    // nine has all of the same symbols as four, but 6,0 will become 8
                } else if signal.union(&four).count() == signal.len() {
                    nine = Some(signal);
                } else {
                    zero = Some(signal);
                }
            }
            val => panic!("Looks like we missed a case??? {}", val),
        }
    }
    let zero = zero.unwrap();
    let two = two.unwrap();
    let three = three.unwrap();
    let five = five.unwrap();
    let six = six.unwrap();
    let nine = nine.unwrap();
    let num = digits
        .iter()
        .map(|digit_signal| match digit_signal {
            digit if digit == zero => '0',
            digit if digit == &one => '1',
            digit if digit == two => '2',
            digit if digit == three => '3',
            digit if digit == &four => '4',
            digit if digit == five => '5',
            digit if digit == six => '6',
            digit if digit == &seven => '7',
            digit if digit == &eight => '8',
            digit if digit == nine => '9',

            _ => panic!("Didn't match any digits???"),
        })
        .collect::<String>()
        .parse()
        .unwrap();
    num
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./test-input.txt");
    #[test]
    fn part_2() {
        assert_eq!(sum_outputs(INPUT), 61229);
    }
}
