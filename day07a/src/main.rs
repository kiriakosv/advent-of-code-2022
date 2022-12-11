use std::collections::HashMap;

/*
This is a suboptimal but fun approach. Instead of creating a tree data structure
I maintain a list of known directory names and a directory stack. I solve the
problem of same name directories by suffixing with an incrementing number.
I may come back with a more textbook approach.
*/
fn main() {
    let mut deduplicator = 0;
    let terminal_lines = include_str!("../input.txt").split_terminator('\n');

    let mut directory_stack: Vec<String> = vec![];
    let mut known_directories: Vec<String> = vec![];
    let mut directory_sizes: HashMap<String, u32> = HashMap::new();

    terminal_lines.for_each(|line| {
        if line.starts_with("$ cd") {
            let directory = String::from(line.split(' ').last().unwrap());

            if directory == ".." {
                directory_stack.pop();
            } else if known_directories.contains(&directory) {
                let deduplicated_directory = format!("{}{}", directory, deduplicator);
                directory_stack.push(deduplicated_directory.clone());
                known_directories.push(deduplicated_directory);

                deduplicator += 1;
            } else {
                directory_stack.push(directory.clone());
                known_directories.push(directory);
            }
        } else if !line.starts_with("dir") && !line.starts_with("$ ls") {
            let file_size: u32 = line.split_once(' ').unwrap().0.parse().unwrap();

            directory_stack.iter().for_each(|directory| {
                let size = directory_sizes.entry(directory.clone()).or_insert(0);
                *size += file_size;
            })
        }
    });

    let result: u32 = directory_sizes.values().filter(|v| **v <= 100_000).sum();
    println!("{}", result);
}
