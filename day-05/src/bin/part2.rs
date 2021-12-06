use std::cmp::max;
use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{i16, newline, space1},
    combinator::opt,
    multi::separated_list1,
    IResult,
};
use std::collections::BTreeMap;

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    let num_intersections = count_intersections(&input);
    println!("Number of intersections: {}", num_intersections);
}

fn count_intersections(input: &str) -> usize {
    let (_, line_list) = line2d_list(input).unwrap();
    let mut intersections: BTreeMap<Point2D, i8> = BTreeMap::new();
    line_list
        .iter()
        .for_each(|line| line.plot_points(&mut intersections));
    intersections
        .iter()
        .filter(|(_, &count)| count >= 2)
        .count()
}

fn line_2d(input: &str) -> IResult<&str, Line2D> {
    let (input, start) = point2d(input)?;
    let (input, _) = point_separator(input)?;
    let (input, end) = point2d(input)?;
    Ok((input, Line2D::new(start, end)))
}

fn point2d(input: &str) -> IResult<&str, Point2D> {
    let (input, x) = i16(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = i16(input)?;
    Ok((input, Point2D { x, y }))
}

fn point_separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = opt(space1)(input)?;
    let (input, _) = opt(tag("-"))(input)?;
    let (input, _) = opt(tag(">"))(input)?;
    let (input, _) = opt(space1)(input)?;
    Ok((input, ()))
}

fn line2d_list(input: &str) -> IResult<&str, Vec<Line2D>> {
    let (input, line_list) = separated_list1(newline, line_2d)(input)?;
    Ok((input, line_list))
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd)]
struct Point2D {
    x: i16,
    y: i16,
}

#[derive(Debug, Clone, Copy)]
struct Line2D {
    start: Point2D,
    end: Point2D,
    orientation: Orientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Line2D {
    fn new(start: Point2D, end: Point2D) -> Self {
        let dx: i16 = (end.x - start.x).try_into().unwrap();
        let dy: i16 = (end.y - start.y).try_into().unwrap();
        if dx == 0 {
            Line2D {
                start,
                end,
                orientation: Orientation::Vertical,
            }
        } else if dy == 0 {
            Line2D {
                start,
                end,
                orientation: Orientation::Horizontal,
            }
        } else {
            Line2D {
                start,
                end,
                orientation: Orientation::Diagonal,
            }
        }
    }
    fn plot_points(&self, point_matrix: &mut BTreeMap<Point2D, i8>) {
        let dx: i16 = self.end.x - self.start.x;
        let dy: i16 = self.end.y - self.start.y;
        for i in 0..=max(dx.abs(), dy.abs()) {
            let x = self.start.x + i * dx.signum();
            let y = self.start.y + i * dy.signum();
            point_matrix
                .entry(Point2D { x, y })
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part_2() {
        assert_eq!(count_intersections(INPUT), 12);
    }
}
