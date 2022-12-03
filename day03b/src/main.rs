use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let priority_map: HashMap<char, i32> = ('a'..='z')
        .map(char::from)
        .zip(1..=26)
        .chain(('A'..='Z').map(char::from).zip(27..=52))
        .collect();

    let result: i32 = include_str!("../input.txt")
        .split_terminator('\n')
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|r| {
            let group_rucksack = r
                .iter()
                .map::<Vec<char>, _>(|i| i.chars().sorted_unstable().dedup().collect())
                .concat();

            let mut occurences: HashMap<char, i32> = HashMap::new();

            for i in group_rucksack {
                let count = occurences.entry(i).or_insert(0);

                *count += 1;
            }

            let badge = occurences.iter().max_by_key(|e| e.1).unwrap().0;

            *priority_map.get(badge).unwrap()
        })
        .sum();

    println!("{:?}", result)
}
