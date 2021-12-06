use std::cmp::max;
use std::fs;

use ndarray::Array2;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline, space1},
    combinator::opt,
    multi::separated_list1,
    IResult,
};

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

fn count_intersections(input: &str) -> i32 {
    let (_, line_list) = line2d_list(input).unwrap();
    let line_list: Vec<Line2D> = line_list
        .into_iter()
        .filter(|line| line.orientation != Orientation::Other)
        .collect();
    let max_point: Point2D = line_list
        .iter()
        .filter(|line| line.orientation != Orientation::Other)
        .fold(Point2D::default(), |max_point, curr_line| Point2D {
            x: max(max(curr_line.start.x, curr_line.end.x), max_point.x),
            y: max(max(curr_line.start.y, curr_line.end.y), max_point.y),
        });
    let mut intersections = Array2::<i32>::zeros((
        TryInto::<usize>::try_into(max_point.x).unwrap() + 1usize,
        TryInto::<usize>::try_into(max_point.y).unwrap() + 1usize,
    ));

    line_list
        .iter()
        .for_each(|line| line.plot_points(&mut intersections));
    intersections.fold(0, |mut num_intersections, val| {
        if *val >= 2 {
            num_intersections += 1;
        }
        num_intersections
    })
}

fn line_2d(input: &str) -> IResult<&str, Line2D> {
    let (input, start) = point2d(input)?;
    let (input, _) = point_separator(input)?;
    let (input, end) = point2d(input)?;
    Ok((input, Line2D::new(start, end)))
}

fn point2d(input: &str) -> IResult<&str, Point2D> {
    let (input, x) = i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = i32(input)?;
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

#[derive(Debug, Clone, Copy, Default)]
struct Point2D {
    x: i32,
    y: i32,
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
    Other,
}

impl Line2D {
    fn new(start: Point2D, end: Point2D) -> Self {
        if start.x == end.x {
            Line2D {
                start,
                end,
                orientation: Orientation::Vertical,
            }
        } else if start.y == end.y {
            Line2D {
                start,
                end,
                orientation: Orientation::Horizontal,
            }
        } else {
            Line2D {
                start,
                end,
                orientation: Orientation::Other,
            }
        }
    }
    fn plot_points(&self, point_matrix: &mut Array2<i32>) {
        if self.orientation == Orientation::Vertical {
            let (start_y, end_y) = {
                if self.start.y < self.end.y {
                    (
                        self.start.y.try_into().unwrap(),
                        self.end.y.try_into().unwrap(),
                    )
                } else {
                    (
                        self.end.y.try_into().unwrap(),
                        self.start.y.try_into().unwrap(),
                    )
                }
            };
            for y_coord in start_y..=end_y {
                point_matrix[(y_coord, self.start.x.try_into().unwrap())] += 1;
            }
        } else {
            let (start_x, end_x): (usize, usize) = {
                if self.start.x < self.end.x {
                    (
                        self.start.x.try_into().unwrap(),
                        self.end.x.try_into().unwrap(),
                    )
                } else {
                    (
                        self.end.x.try_into().unwrap(),
                        self.start.x.try_into().unwrap(),
                    )
                }
            };
            for x_coord in start_x..=end_x {
                point_matrix[(self.start.y.try_into().unwrap(), x_coord)] += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part_1() {
        assert_eq!(count_intersections(INPUT), 5);
    }
}
