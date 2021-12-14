use std::collections::BTreeSet;

use ndarray::Array2;
use nom::{IResult, bytes::complete::{take_until, tag}, sequence::{tuple, separated_pair}, character::complete::{newline, u16, one_of}, multi::separated_list1, combinator::opt};



pub fn input_instructions(input: &str) -> IResult<&str, (BTreeSet<(u16,u16)>, Vec<Instruction>)> {
    let (input, point_vec) = points(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, instructions) = separated_list1(newline, instructions)(input)?;
    let point_map = point_vec.into_iter().collect::<BTreeSet<_>>();
    Ok((input, (point_map, instructions)))
}

pub fn points(input: &str) -> IResult<&str, Vec<(u16, u16)>> {
    let (input, points) = take_until("\n\n")(input)?;
    let (_, point_vec) = separated_list1(newline, separated_pair(u16, tag(","), u16))(points)?;

    Ok((input, point_vec))
}

pub fn instructions(input: &str) -> IResult<&str, Instruction> {
    let (input,_) = tag("fold along ")(input)?;
    let (input, (direction, coordinate)) = separated_pair(one_of("xy"), tag("="), u16)(input)?; 
    match direction {
        'x' => Ok((input, Instruction::X(coordinate))),
        'y' => Ok((input, Instruction::Y(coordinate))),
        _ => panic!("Issue parsing coordinates")
    }
}

pub fn create_matrix(points: Vec<(u16, u16)>) -> Array2<Option<u8>> {
    let max_y = (points.iter().max_by_key(|(_,y)| y).unwrap().1) as usize;
    let max_x = (points.iter().max_by_key(|(x,_)| x).unwrap().0) as usize;
    
    let mut initial_matrix: Array2<Option<u8>> = Array2::from_elem((max_y + 1, max_x + 1), None);
    for (x,y) in points {
        let val =initial_matrix.get_mut((y as usize, x as usize)).unwrap();
        *val = Some(1);
    }
    initial_matrix
}


pub enum Instruction {
    X(u16),
    Y(u16)
}