

use nom::{bytes::complete::tag, character::complete::u16, multi::separated_list1, IResult};

pub fn crab_positions(input: &str) -> IResult<&str, Vec<u16>> {
    let (input, positions) = separated_list1(tag(","), u16)(input)?;

    Ok((input, positions))
}
