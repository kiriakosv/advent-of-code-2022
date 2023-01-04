fn main() {
    let motions: Vec<&str> = include_str!("../test_input.txt")
        .split_terminator('\n')
        .fold(vec![], |mut motions, line| {
            let (a, b) = line.split_once(' ').unwrap();
            motions.push(vec![a; b.parse::<usize>().unwrap()]);
            motions
        })
        .into_iter()
        .flatten()
        .collect();

    println!("{:?}", motions);
}
