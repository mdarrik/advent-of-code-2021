use std::{fs, collections::{BTreeMap, BTreeSet}};
use itertools::Itertools;

use day_17::parse_input;

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use nom::character::complete::newline;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("min risk: {}", part_2(&input));
}

fn part_2(input: &str) -> usize {
    let (_, target_area) = parse_input(input).unwrap();

    let min_y = target_area.y.start();

    // the absolute max y velocity you can reach and still be in range (because after crossing zero the negative velocity will be whatever you started with + 1)
    let max_y_velocity = min_y.abs() as i32 - 1;

    // min_x_velocity. Should  be ~ 2 * sqrt(target_area.min * 2);

    let mut min_x_velocity = f32::sqrt(*target_area.x.start() as f32 * 2.0) as i32;
    
    let mut max_x_distance_from_velocity = (min_x_velocity * (min_x_velocity  + 1))/2;
    assert_eq!(true, target_area.x.contains(&max_x_distance_from_velocity));
    loop {
        let new_max_x = max_x_distance_from_velocity - min_x_velocity;
        if target_area.x.contains(&new_max_x) {
            min_x_velocity = min_x_velocity -1;
            max_x_distance_from_velocity = new_max_x
        } else {
            break;
        }

    }

    let mut valid_points = BTreeSet::new();

    for (start_x,start_y) in (min_x_velocity..=*target_area.x.end()).cartesian_product(*min_y..=max_y_velocity) {
        let mut position_x = 0;
        let mut velocity_x = start_x;
        let mut position_y = 0;
        let mut velocity_y = start_y;

        while position_x < *target_area.x.end() && position_y > *target_area.y.start() {
            position_x += velocity_x;
            position_y += velocity_y;
            match velocity_x.signum() {
                -1 => velocity_x += 1,
                1 => velocity_x -= 1,
                0 => (),
                _ => panic!("signum should always be -1, 1, 0")
            }
            velocity_y -= 1;
            if target_area.x.contains(&position_x) && target_area.y.contains(&position_y) {
                valid_points.insert((start_x, start_y));
            }
        }
    }

    // let mut step_x_map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    // let mut step_y_map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    // for start_x in  min_x_velocity..=*target_area.x.end() {
    //     let mut position = 0;
    //     let mut velocity = start_x;
    //     let mut step = 0;
    //     while position < *target_area.x.end() {
    //         step +=1;
    //         position += velocity;
    //         match velocity.signum() {
    //             -1 => velocity += 1,
    //             1 => velocity -= 1,
    //             0 => (),
    //             _ => panic!()
    //         }
    //         if target_area.x.contains(&position) {
    //             step_x_map.entry(step).and_modify(|position_vec| position_vec.push(start_x)).or_insert_with(|| vec![start_x]);
    //             break;
    //         }
    //     }
    // }

    // for start_y in dbg!(*min_y..=max_y_velocity) {
    //     let mut position = 0;
    //     let mut velocity = start_y;
    //     let mut step = 0;

    //     while position > *target_area.y.start() {
    //         step += 1;
    //         position += velocity;
    //         velocity -= 1;
    //         if target_area.y.contains(&position) {
    //             step_y_map.entry(step).and_modify(|position_vec| position_vec.push(start_y)).or_insert_with(|| vec![start_y]);
    //             break;
    //         }
    //     }
    // }

    // let mut valid_points: BTreeSet<(i32,i32)> = BTreeSet::new();
    // for (step, x_velocities) in step_x_map {
    //     if let Some(y_velocities) = step_y_map.get(&step) {
    //        for (x,y) in x_velocities.iter().cartesian_product(y_velocities.iter()) {
    //            valid_points.insert((*x,*y));
    //        }
    //     }
    // } 

    valid_points.len()
    
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");

    
    #[test]
    fn test_part_1_input_1() {
        assert_eq!(part_2(&INPUT), 112);
    }
}
