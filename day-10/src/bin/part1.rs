use std::fs;

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("danger rating: {}", score_invalid_parens(&input));
}

fn score_invalid_parens(input: &str) -> u32 {
    input
        .lines()
        .fold(0, |score, line| calc_line_score(line) + score)

    // 0
}

fn calc_line_score(line: &str) -> u32 {
    let mut parens: Vec<char> = vec![];
    let invalid_paren_value = line.chars().find(|paren| {
        match paren {
            '(' | '[' | '{' | '<' => {
                parens.push(*paren);
                false
            }
            ')' | ']' | '}' | '>' => {
                let last_paren = parens.pop();
                if let Some(open_paren) = last_paren {
                    match (open_paren, paren) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => false,
                        // if it doesn't match the invalid parens, it's the one we're looking for
                        _ => true,
                    }
                } else {
                    // if there's no open parens left treat it as an error?
                    true
                }
            }
            _ => panic!("Got an input that wasn't a parenthesis"),
        }
    });
    invalid_paren_value.map_or(0, |invalid_val| match invalid_val {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("something else got returned??"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part1() {
        assert_eq!(score_invalid_parens(INPUT), 26397);
    }
}
