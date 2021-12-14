use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::newline,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<(CaveType, CaveType)>> {
    let (input, edges) =
        separated_list1(newline, separated_pair(node_parse, tag("-"), node_parse))(input)?;
    Ok((input, edges))
}

fn node_parse(input: &str) -> IResult<&str, CaveType> {
    alt((start, end, big_cave_type, small_cave_type))(input)
}

fn is_uppercase(c: char) -> bool {
    c.is_uppercase()
}

fn is_lowercase(c: char) -> bool {
    c.is_lowercase()
}

fn big_cave_type(input: &str) -> IResult<&str, CaveType> {
    let (input, node) = take_while1(is_uppercase)(input)?;
    Ok((input, CaveType::Big(node)))
}

fn small_cave_type(input: &str) -> IResult<&str, CaveType> {
    let (input, node) = take_while1(is_lowercase)(input)?;
    Ok((input, CaveType::Small(node)))
}

fn start(input: &str) -> IResult<&str, CaveType> {
    let (input, _) = tag_no_case("start")(input)?;
    Ok((input, CaveType::Start))
}

pub fn end(input: &str) -> IResult<&str, CaveType> {
    let (input, _) = tag_no_case("end")(input)?;
    Ok((input, CaveType::End))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CaveType<'a> {
    Start,
    End,
    Big(&'a str),
    Small(&'a str),
}
