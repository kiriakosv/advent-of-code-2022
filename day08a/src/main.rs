use std::collections::HashSet;

fn main() {
    let grid: Vec<Vec<i32>> = include_str!("../input.txt")
        .split_terminator('\n')
        .map(|i| i.chars().map(|i| i.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut visible_trees = HashSet::new();

    let grid_size = grid.len();

    for i in 0..grid_size {
        let mut heighest_tree_height: i32 = -1;
        for j in 0..grid_size {
            if grid[i][j] > heighest_tree_height {
                heighest_tree_height = grid[i][j];
                visible_trees.insert(format!("{}-{}", i, j));
            }
        }
    }

    for i in 0..grid_size {
        let mut heighest_tree_height: i32 = -1;
        for j in (0..grid_size).rev() {
            if grid[i][j] > heighest_tree_height {
                heighest_tree_height = grid[i][j];
                visible_trees.insert(format!("{}-{}", i, j));
            }
        }
    }

    for i in 0..grid_size {
        let mut heighest_tree_height: i32 = -1;
        for j in 0..grid_size {
            if grid[j][i] > heighest_tree_height {
                heighest_tree_height = grid[j][i];
                visible_trees.insert(format!("{}-{}", j, i));
            }
        }
    }

    for i in 0..grid_size {
        let mut heighest_tree_height: i32 = -1;
        for j in (0..grid_size).rev() {
            if grid[j][i] > heighest_tree_height {
                heighest_tree_height = grid[j][i];
                visible_trees.insert(format!("{}-{}", j, i));
            }
        }
    }

    println!("{}", visible_trees.len());
}
