fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split_terminator("\n\n")
            .map::<u32, _>(|i| i
                .split_terminator('\n')
                .map(|j| j.parse::<u32>().unwrap())
                .sum())
            .max()
            .unwrap()
    );
}
