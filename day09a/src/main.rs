use std::collections::HashSet;

fn main() {
    const NUM_OF_KNOTS: usize = 2;
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); NUM_OF_KNOTS];

    let result = include_str!("../input.txt")
        .split_terminator('\n')
        .fold(vec![], |mut motions, line| {
            let (a, b) = line.split_once(' ').unwrap();
            motions.push(vec![a; b.parse::<usize>().unwrap()]);
            motions
        })
        .into_iter()
        .flatten()
        .fold(HashSet::new(), |mut tail_locations, motion| {
            match motion {
                "U" => knots[0].0 += 1,
                "D" => knots[0].0 -= 1,
                "R" => knots[0].1 += 1,
                "L" => knots[0].1 -= 1,
                _ => panic!(),
            }

            for i in 1..knots.len() {
                if (knots[i - 1].0 - knots[i].0).abs() >= 2
                    || (knots[i - 1].1 - knots[i].1).abs() >= 2
                {
                    knots[i].0 += (knots[i - 1].0 - knots[i].0).signum();
                    knots[i].1 += (knots[i - 1].1 - knots[i].1).signum();
                }
            }

            tail_locations.insert(knots[knots.len() - 1]);
            tail_locations
        })
        .len();

    println!("{:?}", result);
}
