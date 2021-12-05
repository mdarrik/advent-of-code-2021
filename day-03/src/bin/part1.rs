use std::fs;

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};


#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./p1.input.txt").unwrap();
    let p1_output = p1(input.clone());

    println!("p1: {}", p1_output);
}

fn p1(input: String) -> i32 {
    let lines = input.lines();
    let number_of_inputs: u32 = input.clone().lines().count().try_into().unwrap();
    let bit_counts = lines.fold(Vec::<u32>::new(), |mut bits, line| {
        for (index, bit) in line.char_indices() {
            if let Some(bit_num) = bit.to_digit(2) {
                if bits.len() <= index {
                    bits.push(bit_num);
                } else {
                    bits[index] += bit_num;
                }
            }
        }
        bits
    });
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();
    for bit_val in bit_counts {
        if bit_val <= (number_of_inputs / 2) {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    let gamma_decimal =
        i32::from_str_radix(&gamma_rate.into_iter().collect::<String>(), 2).unwrap();
    let epsilon_decimal =
        i32::from_str_radix(&epsilon_rate.into_iter().collect::<String>(), 2).unwrap();
    gamma_decimal * epsilon_decimal
}

#[test]
fn part_1_example() {
    let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#
        .to_string();
    assert_eq!(p1(input), 198);
}
