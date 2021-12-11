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
    println!("danger rating: {}", score_incomplete_parens(&input));
}

fn score_incomplete_parens(input: &str) -> u64 {
    let mut line_scores = input
        .lines()
        .filter_map(calc_line_score)
        .collect::<Vec<u64>>();
    line_scores.sort_unstable();
    *line_scores.get(line_scores.len() / 2).unwrap()
}

fn calc_line_score(line: &str) -> Option<u64> {
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
    if let Some(_) = invalid_paren_value {
        None
    } else if parens.len() == 0 {
        None
    } else {
        parens.reverse();
        Some(parens.iter().fold(0u64, |mut score, paren| {
            score = score * 5;
            match paren {
                '(' => score + 1,
                '[' => score + 2,
                '{' => score + 3,
                '<' => score + 4,
                _ => panic!("{} left over", paren),
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = include_str!("./test-input.txt");
    #[test]
    fn part2() {
        assert_eq!(score_incomplete_parens(INPUT), 288957);
    }
}
