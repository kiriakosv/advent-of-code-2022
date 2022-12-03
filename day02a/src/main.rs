use std::collections::HashMap;

fn main() {
    let scores: HashMap<&str, i32> = [
        ("B X", 1),
        ("C Y", 2),
        ("A Z", 3),
        ("A X", 4),
        ("B Y", 5),
        ("C Z", 6),
        ("C X", 7),
        ("A Y", 8),
        ("B Z", 9),
    ]
    .into_iter()
    .collect();

    let game: i32 = include_str!("../input.txt")
        .split_terminator('\n')
        .map::<&i32, _>(|v| scores.get(v).unwrap())
        .sum();

    println!("{}", game)
}
