fn main() {
    let mut v = include_str!("../input.txt")
        .split_terminator("\n\n")
        .map::<u32, _>(|i| {
            i.split_terminator('\n')
                .map(|j| j.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();

    v.sort_unstable();
    v.reverse();
    v.truncate(3);

    println!("{:?}", v.into_iter().sum::<u32>())
}
