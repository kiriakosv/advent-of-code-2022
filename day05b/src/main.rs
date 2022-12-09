fn main() {
    // This is ugly. I'll come back later to construct the stacks from the input file.

    // let mut stacks = [
    //     vec!['Z', 'N'],
    //     vec!['M', 'C', 'D'],
    //     vec!['P']];

    let mut stacks = [
        vec!['Z', 'P', 'M', 'H', 'R'],
        vec!['P', 'C', 'J', 'B'],
        vec!['S', 'N', 'H', 'G', 'L', 'C', 'D'],
        vec!['F', 'T', 'M', 'D', 'Q', 'S', 'R', 'L'],
        vec!['F', 'S', 'P', 'Q', 'B', 'T', 'Z', 'M'],
        vec!['T', 'F', 'S', 'Z', 'B', 'G'],
        vec!['N', 'R', 'V'],
        vec!['P', 'G', 'L', 'T', 'D', 'V', 'C', 'M'],
        vec!['W', 'Q', 'N', 'J', 'F', 'M', 'L'],
    ];

    let (_, s) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let instructions = s
        .split_terminator('\n')
        .map(|f| {
            let l = f.split_terminator(' ').collect::<Vec<&str>>();
            vec![l[1], l[3], l[5]]
                .iter()
                .map(|j| j.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    instructions.iter().for_each(|instruction| {
        let number_of_crates = *instruction.get(0).unwrap() as usize;
        let from = (*instruction.get(1).unwrap() - 1) as usize;
        let to = (*instruction.get(2).unwrap() - 1) as usize;

        let from_stack: &mut Vec<char> = &mut stacks[from];
        let to_be_moved = &mut from_stack.split_off(from_stack.len() - number_of_crates);
        let to_stack: &mut Vec<char> = &mut stacks[to];

        to_stack.append(to_be_moved);
    });

    println!(
        "{:?}",
        stacks.iter().map(|i| i.last().unwrap()).collect::<String>()
    );
}
