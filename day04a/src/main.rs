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
        .filter(|(start1, end1, start2, end2)| contained(*start1, *end1, *start2, *end2))
        .count();

    println!("{}", result)
}

fn contained(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1)
}
