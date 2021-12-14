use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use day_12::{parse_input, CaveType};
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

use petgraph::{ graph::{NodeIndex, UnGraph}};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("number of paths: {}", part_2(&input));
}

fn build_graph_from_edges<'a>(
    edges: Vec<(CaveType<'a>, CaveType<'a>)>,
) -> (UnGraph<CaveType<'a>, ()>, HashMap<CaveType, NodeIndex>) {
    let mut graph = UnGraph::<CaveType, ()>::new_undirected();
    let mut node_indices_map = HashMap::<CaveType, NodeIndex>::new();
    for (first_node, second_node) in edges {
        let (first_node_index, second_node_index) = match (
            node_indices_map.get(&first_node).cloned(),
            node_indices_map.get(&second_node).cloned(),
        ) {
            (None, None) => {
                let first_node_index = graph.add_node(first_node);
                let second_node_index = graph.add_node(second_node);
                node_indices_map.insert(first_node, first_node_index);
                node_indices_map.insert(second_node, second_node_index);
                (first_node_index, second_node_index)
            }
            (None, Some(second_node_index)) => {
                let first_node_index = graph.add_node(first_node);
                node_indices_map.insert(first_node, first_node_index);
                (first_node_index, second_node_index)
            }
            (Some(first_node_index), None) => {
                let second_node_index = graph.add_node(second_node);
                node_indices_map.insert(second_node, second_node_index);
                (first_node_index, second_node_index)
            }
            (Some(first_node_index), Some(second_node_index)) => {
                (first_node_index, second_node_index)
            }
        };
        graph.update_edge(first_node_index, second_node_index, ());
    }
    (graph, node_indices_map)
}

fn part_2(input: &str) -> u32 {
    let (_, edges) = parse_input(input).unwrap();
    let (graph, node_indices_map) = build_graph_from_edges(edges);

    let start_id = *node_indices_map.get(&CaveType::Start).unwrap();

    find_paths(start_id, &mut BTreeMap::new(), &graph, false)
}

fn find_paths<'a>(
    current_node: NodeIndex,
    nodes_visited: &mut BTreeMap<&'a str, Vec<NodeIndex>>,
    graph: &UnGraph<CaveType<'a>, ()>,
    has_visited_small_cave_twice: bool
) -> u32 {
    let mut complete_paths = 0u32;
    for node in graph.neighbors(current_node) {
        match &graph[node] {
            CaveType::Start => (),
            CaveType::End => complete_paths += 1,
            CaveType::Big(cave) => {
                let mut new_visited = nodes_visited.clone();
                new_visited
                    .entry(cave)
                    .and_modify(|path| path.push(node))
                    .or_insert_with(|| vec![node]);
                complete_paths += find_paths(node, &mut new_visited, graph, has_visited_small_cave_twice);
            }
            CaveType::Small(cave) => {
                if !nodes_visited.contains_key(cave) || (nodes_visited.get(cave).unwrap().len() == 1 && !has_visited_small_cave_twice)
                {
                    let mut new_visited = nodes_visited.clone();
                   let val = new_visited
                        .entry(cave)
                        .and_modify(|path| path.push(node))
                        .or_insert_with(|| vec![node]);
                    let new_has_visited_small_cave_twice  = has_visited_small_cave_twice || val.len() == 2;
                    complete_paths += find_paths(node, &mut new_visited.clone(), graph, new_has_visited_small_cave_twice)
                }
            }
        }
    }
    complete_paths
}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALL_INPUT: &'static str = include_str!("./test-input-small.txt");
    const LARGER_INPUT: &'static str = include_str!("./test-input-larger.txt");
    #[test]
    fn part_2_small() {
        assert_eq!(part_2(&SMALL_INPUT), 36);
    }

    #[test]
    fn part_2_larger() {
        assert_eq!(part_2(&LARGER_INPUT), 3509);
    }
}
