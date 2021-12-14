use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline},
    multi::{many_till, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, (Vec<char>, HashMap<(char, char), char>)> {
    let (input, (starting_polymer, _)) = many_till(anychar, newline)(input)?;
    let (input, _) = newline(input)?;
    let (input, rules) = separated_list1(newline, insertion_rule)(input)?;
    let rules_map = HashMap::from_iter(rules.into_iter());
    Ok((input, (starting_polymer, rules_map)))
}

fn insertion_rule(input: &str) -> IResult<&str, ((char, char), char)> {
    let (input, rule) = separated_pair(pair(anychar, anychar), tag(" -> "), anychar)(input)?;
    Ok((input, rule))
}

pub fn build_polymer(input: &str, steps: u16) -> u64 {
    let (_, (mut starting_polymer, rules)) = parse_input(input).unwrap();
    for _ in 0..steps {
        apply_rules(&mut starting_polymer, &rules)
    }
    let element_quantities =
        starting_polymer
            .iter()
            .fold(BTreeMap::new(), |mut element_counts, element| {
                element_counts
                    .entry(element)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                element_counts
            });

    let (_, most_common_count) = element_quantities.iter().max_by_key(|(_, v)| **v).unwrap();
    let (_, least_common_count) = element_quantities.iter().min_by_key(|(_, v)| **v).unwrap();

    most_common_count - least_common_count
}

pub fn build_polymer_2(input: &str, steps: u16) -> u64 {
    let (_, (starting_polymer, rules)) = parse_input(input).unwrap();
    let mut pair_map = rules
        .clone()
        .into_iter()
        .map(|(key, _)| (key, 0))
        .collect::<HashMap<_, _>>();
    for slice in starting_polymer.windows(2) {
        pair_map.entry((slice[0], slice[1])).and_modify(|c| *c += 1);
    }
    for _ in 0..steps {
        let mut new_counts = HashMap::with_capacity(pair_map.len());
        for ((left, right), count) in pair_map.iter().filter(|(_, v)| **v > 0) {
            if let Some(inserted_char) = rules.get(&(*left, *right)) {
                new_counts
                    .entry((*left, *inserted_char))
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
                new_counts
                    .entry((*inserted_char, *right))
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
            }
        }
        pair_map = new_counts;
    }
    let mut element_quantities = pair_map.iter().fold(
        BTreeMap::new(),
        |mut char_counts: BTreeMap<&char, u64>, ((left, right), amount)| {
            char_counts
                .entry(left)
                .and_modify(|count| *count += amount)
                .or_insert(*amount);
            char_counts
                .entry(right)
                .and_modify(|count| *count += amount)
                .or_insert(*amount);
            char_counts
        },
    );
    element_quantities
        .entry(starting_polymer.first().unwrap())
        .and_modify(|c| *c += 1);
    element_quantities
        .entry(starting_polymer.last().unwrap())
        .and_modify(|c| *c += 1);
    let (_, most_common_count) = element_quantities.iter().max_by_key(|(_, v)| **v).unwrap();
    let (_, least_common_count) = element_quantities.iter().min_by_key(|(_, v)| **v).unwrap();
    (most_common_count - least_common_count) / 2
}

fn apply_rules(polymer: &mut Vec<char>, rules: &HashMap<(char, char), char>) {
    let mut rules_to_apply = Vec::with_capacity(polymer.len());
    for (index, pair) in polymer.windows(2).enumerate() {
        if let Some(element) = rules.get(&(pair[0], pair[1])) {
            rules_to_apply.push((index, element));
        }
    }
    let mut num_inserted = 0usize;
    for (index, element) in rules_to_apply {
        polymer.insert(index + num_inserted + 1, *element);
        num_inserted += 1;
    }
}
