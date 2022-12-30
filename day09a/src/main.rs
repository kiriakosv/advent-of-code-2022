fn main() {
    let motions = include_str!("../test_input.txt").split_terminator('\n')
        .map(|line| line.split_once(' ').unwrap()).collect::<Vec<(&str, &str)>>();

    println!("{:?}", motions);
}
