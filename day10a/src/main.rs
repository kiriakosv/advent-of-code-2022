fn main() {
    let register_values_over_time = include_str!("../input.txt")
        .split_terminator('\n')
        .into_iter()
        .fold(vec![1; 1], |mut r: Vec<i32>, instruction| {
            r.push(*r.last().unwrap());

            match instruction {
                "noop" => {}
                _ => {
                    let number = instruction
                        .split_once(' ')
                        .unwrap()
                        .1
                        .parse::<i32>()
                        .unwrap();

                    r.push(*r.last().unwrap() + number)
                }
            }

            r
        });

    let critical_cycles: Vec<i32> = vec![19, 59, 99, 139, 179, 219];
    let result: i32 = critical_cycles
        .into_iter()
        .map(|i| register_values_over_time[i as usize] * (i + 1))
        .sum();

    println!("{:?}", result);
}
