use std::{cmp::min, collections::BTreeMap, fs};

use day_15::parse_input;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use ndarray::Array2;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("score of polymer after 10 steps: {}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    let (_, value_map) = parse_input(input).unwrap();
    let end_point = (value_map.nrows() - 1, value_map.ncols() - 1);
    let mut risk_tree = value_map
        .indexed_iter()
        .map(|(position, _)| (position, u32::MAX))
        .collect::<BTreeMap<(usize, usize), u32>>();
    get_path(&mut risk_tree, (0, 0), 0, &value_map);

    *risk_tree.get(&end_point).unwrap()
}

fn get_path(
    visited_nodes: &mut BTreeMap<(usize, usize), u32>,
    current_point: (usize, usize),
    current_risk: u32,
    value_map: &Array2<u8>,
) {
    let up = current_point.0.saturating_sub(1);
    let down = min(current_point.0 + 1, value_map.nrows() - 1);
    let right = min(current_point.1 + 1, value_map.ncols() - 1);
    let left = current_point.1.saturating_sub(1);
    for point in [
        (up, current_point.1),
        (current_point.0, left),
        (current_point.0, right),
        (down, current_point.1),
    ] {
        let point_weight: u32 = (*value_map.get(point).unwrap()).into();
        if *visited_nodes.entry(point).or_insert(u32::MAX) > point_weight + current_risk {
            visited_nodes
                .entry(point)
                .and_modify(|w| {
                    if *w > point_weight + current_risk {
                        *w = point_weight + current_risk
                    }
                })
                .or_insert(point_weight);
            get_path(visited_nodes, point, current_risk + point_weight, value_map);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&INPUT), 40);
    }
}
