use std::collections::HashMap;

fn main() {
    let scores: HashMap<&str, i32> = [
        ("B X", 1),
        ("C X", 2),
        ("A X", 3),
        ("A Y", 4),
        ("B Y", 5),
        ("C Y", 6),
        ("C Z", 7),
        ("A Z", 8),
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
