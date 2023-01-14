fn main() {
    include_str!("../input.txt")
        .split_terminator('\n')
        .into_iter()
        .fold(vec![1; 1], |mut register_values: Vec<i32>, instruction| {
            register_values.push(*register_values.last().unwrap());

            match instruction {
                "noop" => {}
                _ => {
                    let number = instruction
                        .split_once(' ')
                        .unwrap()
                        .1
                        .parse::<i32>()
                        .unwrap();

                    register_values.push(*register_values.last().unwrap() + number)
                }
            }

            register_values
        })
        .into_iter()
        .enumerate()
        .fold(vec![], |mut rows: Vec<&str>, (pixel, register_value)| {
            if [register_value - 1, register_value, register_value + 1]
                .contains(&(pixel.rem_euclid(40) as i32))
            {
                rows.push("#");
            } else {
                rows.push(".")
            };

            rows
        })
        .chunks(40)
        .for_each(|row| println!("{}", row.join("")));
}
