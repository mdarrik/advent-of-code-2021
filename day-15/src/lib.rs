
use ndarray::Array2;
use nom::{multi::{separated_list1, many1}, IResult, character::complete::{newline, one_of}};

pub fn parse_input(input: &str) -> IResult<&str, Array2<u8>> {
    let num_rows = input.lines().count();
    let (_, values) = separated_list1(newline, many1(one_of("0123456789")))(input)?;
    let num_cols = values.first().unwrap().len();
    let values:Vec<u8> = values.into_iter().flat_map(|r| r.iter().map(|digit| digit.to_digit(10).unwrap() as u8).collect::<Vec<_>>()).collect();
    let out_array = Array2::from_shape_vec((num_rows, num_cols), values).unwrap();
    Ok((input, out_array))
}