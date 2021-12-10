use std::fs;

use day_09::elevation_map;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use ndarray::{s, Array2};
use petgraph::{
    algo::condensation,
    data::Build,
    graph::Node,
    graphmap::{GraphMap, UnGraphMap},
    matrix_graph::MatrixGraph,
    Undirected,
};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("danger rating: {}", find_biggest_basins(&input));
}

fn find_biggest_basins(input: &str) -> u32 {
    let (_, map_as_vec) = elevation_map(input).unwrap();
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();
    let map_of_elevations = Array2::from_shape_vec((num_rows, num_cols), map_as_vec).unwrap();
    let mut graph: UnGraphMap<(usize, usize), ()> = UnGraphMap::new();
    map_of_elevations
        .indexed_iter()
        .filter(|((_, _), val)| **val != 9)
        .for_each(|((row, col), _)| {
            let curr_node = graph.add_node((row, col));
            let top = (row + 1, col);
            let bottom = (row.saturating_sub(1), col);
            let left = (row, col.saturating_sub(1));
            let right = (row, col + 1);
            [top, bottom, left, right].iter().for_each(|node| {
                if let Some(_) = map_of_elevations.get(*node).filter(|v| **v != 9) {
                    let neighbor_node = graph.add_node(*node);
                    graph.update_edge(curr_node, neighbor_node, ());
                }
            })
        });
    let basins = condensation::<(usize, usize), (), Undirected, u32>(graph.into_graph(), false);
    let mut basin_sizes = basins
        .node_weights()
        .map(|node| node.len() as u32).collect::<Vec<u32>>();
    basin_sizes.sort_unstable();
    basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part_1() {
        assert_eq!(find_biggest_basins(&INPUT), 1134);
    }
}
