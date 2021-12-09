use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take_until1, take_while},
    character::complete::{alpha0, alpha1, newline, space0, space1},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

pub fn input_signals_part_1(input: &str) -> IResult<&str, Vec<&str>> {
    let signals = input
        .lines()
        .flat_map(|line| {
            dbg!(&line);
            let (_, signal) = number_signals_part_1(line).unwrap();
            signal
        })
        .collect();
    Ok((input, signals))
}

fn display(input: &str) -> IResult<&str, Vec<&str>> {
    Ok((input, vec![]))
}

fn number_signals_part_1(input: &str) -> IResult<&str, Vec<&str>> {
    // // can discard the first chunk of each line b/c it's not needed for part 1
    let (input, _) = take_until1("|")(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space0(input)?;

    let (input, _) = space0(input)?;
    separated_list0(space1, alpha0)(input)
}

pub fn input_signals_line_part_2(
    input: &str,
) -> IResult<&str, (Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    separated_pair(digit_signal, tag(" | "), digit_signal)(input)
}

fn digit_signal(input: &str) -> IResult<&str, Vec<HashSet<char>>> {
    let (input, result) = separated_list1(space1, alpha1)(input)?;
    let set = result
        .iter()
        .map(|s| HashSet::from_iter(s.chars()))
        .collect();
    Ok((input, set))
}

pub fn parse_inputs_part_2(
    input: &str,
) -> IResult<&str, Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>> {
    Ok(separated_list1(newline, input_signals_line_part_2)(input).unwrap())
}
