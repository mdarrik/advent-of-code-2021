use std::ops::RangeInclusive;

use nom::{IResult, bytes::complete::tag, sequence::separated_pair, character::complete::{one_of, i16}};




pub fn parse_input(input:&str)-> IResult<&str,TargetArea> {
    let (input, _) = tag("target area: ")(input)?;
    let (input, (x_range, y_range)) = separated_pair(range, tag(", "), range)(input)?;
    Ok((input, TargetArea { x: x_range, y: y_range}))
}

fn range(input: &str) -> IResult<&str, RangeInclusive<i32>> {
    let (input, _) = one_of("xy")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, (min_val, max_val)) = separated_pair(i16, tag(".."), i16)(input)?;
    Ok((input, min_val as i32..=max_val as i32))
}


pub struct TargetArea {
   pub x: RangeInclusive<i32>,
   pub y: RangeInclusive<i32>
}