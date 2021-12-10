use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

pub fn elevation_map(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, numbers) = separated_list1(newline, digit1)(input)?;
    let out_vec = numbers
        .iter()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();
    Ok((input, out_vec))
}
