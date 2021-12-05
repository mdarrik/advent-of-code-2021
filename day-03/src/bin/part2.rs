
use std::fs;

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use ndarray::{Array2};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./p1.input.txt").unwrap();
    let p1_output = p2(input.clone());

    println!("p2: {}", p1_output);
}

fn p2(input: String) -> i32 {
    let line_vec: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut input_vec = vec![];
    for line in line_vec.clone() {
        let row: Vec<u32> = line.chars().map(|char| char.to_digit(2).unwrap()).collect();
        input_vec.extend_from_slice(&row);
    }
    let input_matrix =
        Array2::from_shape_vec((line_vec.len(), line_vec[0].len()), input_vec).unwrap();
    let (o2_row, co2_row) = find_sensor_bits(input_matrix);
    let o2_int = i32::from_str_radix(&o2_row, 2).unwrap();
    let co2_int = i32::from_str_radix(&co2_row, 2).unwrap();
    co2_int * o2_int
}

fn find_sensor_bits(input_matrix: Array2<u32>) -> (String, String) {
    let mut removed_o2_indices = vec![];
    let mut removed_co2_indices: Vec<usize> = vec![];
    let mut o2_row_maybe = None;
    let mut co2_row_maybe: Option<String> = None;
    let num_rows = input_matrix.nrows();
    for col_index in 0..input_matrix.ncols() {
        if num_rows - removed_o2_indices.len() > 1 {
            process_column_for_filter(
                col_index,
                &input_matrix,
                &mut removed_o2_indices,
                Sensor::O2,
            );
        } else if o2_row_maybe.is_none() {
            let last_row_index = (0..input_matrix.nrows())
                .find(|index| !removed_o2_indices.contains(index))
                .unwrap();
            o2_row_maybe = Some(
                input_matrix
                    .row(last_row_index)
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
            );
        }

        if num_rows - removed_co2_indices.len() > 1 {
            process_column_for_filter(
                col_index,
                &input_matrix,
                &mut removed_co2_indices,
                Sensor::Co2,
            )
        } else if co2_row_maybe.is_none() {
            let last_row_index = (0..input_matrix.nrows())
                .find(|index| !removed_co2_indices.contains(index))
                .unwrap();
            co2_row_maybe = Some(
                input_matrix
                    .row(last_row_index)
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
            );
        }

        if o2_row_maybe.is_some() && co2_row_maybe.is_some() {
            break;
        }
    }
    match (o2_row_maybe, co2_row_maybe) {
        (Some(o2_row), Some(co2_row)) => (o2_row, co2_row),
        (Some(o2_row), None) => {
            let last_row_index = (0..input_matrix.nrows())
                .find(|index| !removed_co2_indices.contains(index))
                .unwrap();
            let co2_row = input_matrix
                .row(last_row_index)
                .iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join("");
            (o2_row, co2_row)
        }
        (None, Some(co2_row)) => {
            let last_row_index = (0..input_matrix.nrows())
                .find(|index| !removed_o2_indices.contains(index))
                .unwrap();
            let o2_row = input_matrix
                .row(last_row_index)
                .iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join("");
            (o2_row, co2_row)
        }
        (None, None) => {
            let last_o2_row_index = (0..input_matrix.nrows())
                .find(|index| !removed_o2_indices.contains(index))
                .unwrap();
            let o2_row = input_matrix
                .row(last_o2_row_index)
                .iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join("");

            let last_row_index = (0..input_matrix.nrows())
                .find(|index| !removed_co2_indices.contains(index))
                .unwrap();
            let co2_row = input_matrix
                .row(last_row_index)
                .iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join("");
            (o2_row, co2_row)
        }
    }
}

fn process_column_for_filter(
    col_index: usize,
    input_matrix: &Array2<u32>,
    removed_indices: &mut Vec<usize>,
    sensor_type: Sensor,
) {
    let all_cols = input_matrix.column(col_index);
    let cols: Vec<(usize, &u32)> = all_cols
        .iter()
        .enumerate()
        .filter(|(index, _)| !removed_indices.contains(index))
        .collect();
    let col_filter = {
        let sum: u32 = cols.iter().fold(0, |sum, (_, val)| sum + **val);
        let col_length: u32 = cols.len().try_into().unwrap();
        if sum >= (col_length + (col_length % 2)) / 2 {
            match sensor_type {
                Sensor::O2 => 1u32,
                Sensor::Co2 => 0u32,
            }
        } else {
            match sensor_type {
                Sensor::O2 => 0u32,
                Sensor::Co2 => 1u32,
            }
        }
    };
    for (row_index, row_val) in cols.iter() {
        if row_val != &&col_filter {
            removed_indices.push(*row_index)
        }
    }
}

enum Sensor {
    O2,
    Co2,
}

#[test]
fn part_2_example() {
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
    assert_eq!(p2(input), 230);
}
