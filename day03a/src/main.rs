use std::collections::{HashMap, HashSet};

fn main() {
    let priority_map: HashMap<char, i32> = (b'a'..=b'z')
        .map(char::from)
        .zip(1..=26)
        .chain((b'A'..=b'Z').map(char::from).zip(27..=52))
        .collect();

    println!(
        "{:?}",
        include_str!("../input.txt")
            .split_terminator('\n')
            .map::<i32, _>(|c| {
                let (first_half, last_half) = c.split_at(c.len() / 2);
                let set1: HashSet<char> = first_half.chars().collect();
                let set2: HashSet<char> = last_half.chars().collect();

                let c = *set1.intersection(&set2).next().unwrap();

                *priority_map.get(&c).unwrap()
            })
            .sum::<i32>()
    )
}
