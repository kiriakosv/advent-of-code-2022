fn main() {
    let result = include_str!("../input.txt")
        .split_terminator('\n')
        .map(|s| {
            let (s1, s2) = s.split_once(',').unwrap();

            let ((start1, end1), (start2, end2)) =
                (s1.split_once('-').unwrap(), s2.split_once('-').unwrap());

            (
                start1.parse::<u32>().unwrap(),
                end1.parse::<u32>().unwrap(),
                start2.parse::<u32>().unwrap(),
                end2.parse::<u32>().unwrap(),
            )
        })
        .filter(|(start1, end1, start2, end2)| overlaps(*start1, *end1, *start2, *end2))
        .count();

    println!("{}", result)
}

fn overlaps(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    !(end1 < start2 || start1 > end2)
}
