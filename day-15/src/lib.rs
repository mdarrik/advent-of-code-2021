use std::{
    cmp::{min, Reverse},
    collections::HashMap,
};

use ndarray::Array2;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub trait GetU16<T> {
    fn get_u16(&self, pair: (u16, u16)) -> Option<&T>;
}

impl<T> GetU16<T> for Array2<T> {
    fn get_u16(&self, (row, col): (u16, u16)) -> Option<&T> {
        self.get((row as usize, col as usize))
    }
}

fn get_neighbors((row, col): (u16, u16), nrows: u16, ncols: u16) -> [(u16, u16); 4] {
    [
        (row.saturating_sub(0), col),
        (min(row + 1, nrows - 1), col),
        (row, col.saturating_sub(1)),
        (row, min(col + 1, ncols - 1)),
    ]
}

pub fn shortest_distance(
    start_point: (u16, u16),
    value_map: &Array2<u8>,
    end_point: (u16, u16),
) -> u32 {
    let mut visited_nodes = HashMap::with_capacity(value_map.nrows() * value_map.ncols());
    let mut nodes_to_visit =
        std::collections::BinaryHeap::with_capacity(value_map.nrows() + value_map.ncols());
    nodes_to_visit.push(Reverse((0, start_point)));

    let nrows = value_map.nrows() as u16;
    let ncols = value_map.ncols() as u16;

    while let Some(Reverse((current_risk, current_point))) = nodes_to_visit.pop() {
        if *visited_nodes.get(&current_point).unwrap_or(&u32::MAX) < current_risk {
            continue;
        }
        if current_point == end_point {
            return current_risk;
        }
        for neighbor in get_neighbors(current_point, nrows, ncols) {
            let neighbor_weight: u32 = (*value_map.get_u16(neighbor).unwrap()).into();
            let new_weight = neighbor_weight + current_risk;
            if new_weight < *visited_nodes.entry(neighbor).or_insert(u32::MAX) {
                visited_nodes
                    .entry(neighbor)
                    .and_modify(|w| *w = new_weight);
                if neighbor != end_point {
                    nodes_to_visit.push(Reverse((new_weight, neighbor)));
                } else {
                    return new_weight;
                }
            }
        }
    }
    unreachable!("Should get the end point before this")
}

pub fn parse_input(input: &str) -> IResult<&str, Array2<u8>> {
    let num_rows = input.lines().count();
    let (_, values) = separated_list1(newline, many1(one_of("0123456789")))(input)?;
    let num_cols = values.first().unwrap().len();
    let values: Vec<u8> = values
        .into_iter()
        .flat_map(|r| {
            r.iter()
                .map(|digit| digit.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect();
    let out_array = Array2::from_shape_vec((num_rows, num_cols), values).unwrap();
    Ok((input, out_array))
}
