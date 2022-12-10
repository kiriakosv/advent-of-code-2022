use std::collections::HashSet;

fn main() {
    const CONSECUTIVE: usize = 4;

    let result = include_str!("../input.txt")
        .trim_end_matches('\n')
        .chars()
        .collect::<Vec<char>>()
        .windows(CONSECUTIVE)
        .into_iter()
        .position(|candidate| candidate.iter().collect::<HashSet<_>>().len() == CONSECUTIVE)
        .unwrap()
        + CONSECUTIVE;

    println!("{:?}", result);
}
