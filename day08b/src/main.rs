fn main() {
    let grid: Vec<Vec<u32>> = include_str!("../input.txt")
        .split_terminator('\n')
        .map(|i| i.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect();

    let mut high_score: u32 = 0;
    let grid_size = grid.len();

    for i in 0..grid_size {
        for j in 0..grid_size {
            let right = &grid[i][j + 1..];
            let left = &grid[i][..j];
            let down = &grid.iter().map(|x| x[j]).collect::<Vec<u32>>()[i + 1..];
            let up = &grid.iter().map(|x| x[j]).collect::<Vec<u32>>()[..i];

            let element = grid[i][j];

            let right_score = match right.iter().position(|x| *x >= element) {
                Some(i) => i + 1,
                None => right.len(),
            };

            let left_score = match left.iter().rev().position(|x| *x >= element) {
                Some(i) => i + 1,
                None => left.len(),
            };

            let down_score = match down.iter().position(|x| *x >= element) {
                Some(i) => i + 1,
                None => down.len(),
            };

            let up_score = match up.iter().rev().position(|x| *x >= element) {
                Some(i) => i + 1,
                None => up.len(),
            };

            let current_score = (right_score * left_score * down_score * up_score) as u32;
            if current_score > high_score {
                high_score = current_score;
            }
        }
    }

    println!("{}", high_score);
}
