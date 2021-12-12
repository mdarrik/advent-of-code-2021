use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list0},
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Octopus>> {
    let (input, out) = separated_list0(newline, many1(one_of("0123456789")))(input)?;
    let out = out
        .iter()
        .flat_map(|v| {
            v.iter()
                .map(|c| Octopus::Power(c.to_digit(10).unwrap() as u8))
        })
        .collect::<Vec<Octopus>>();

    Ok((input, out))
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Octopus {
    NeedsToFlash,
    Flashed,
    Power(u8),
}

impl std::fmt::Display for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Octopus::NeedsToFlash => f.write_str("NeedsToFlash"),
            Octopus::Flashed => f.write_str("Flashed"),
            Octopus::Power(ref inner) => ::std::fmt::Display::fmt(inner, f),
        }
    }
}

impl Octopus {
    pub fn increment_flash(&mut self) {
        match self {
            Octopus::Power(val) if *val == 9 => *self = Octopus::NeedsToFlash,
            Octopus::Power(val) => *val += 1,
            _ => (),
        };
    }

    pub fn expend_flash(&mut self) {
        if *self == Octopus::NeedsToFlash {
            *self = Octopus::Flashed
        }
    }
    pub fn needs_to_flash(&self) -> bool {
        *self == Octopus::NeedsToFlash
    }
}
