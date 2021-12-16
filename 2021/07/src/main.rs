use day07::{part1_naive, part2_naive};

fn main() {
    let crabs: Vec<u16> = include_str!("../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("Fuel (Part 1): {:8}", part1_naive(&crabs));
    println!("Fuel (Part 2): {:8}", part2_naive(&crabs));
}
