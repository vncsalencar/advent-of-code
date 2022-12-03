use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let priorities_by_char: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, char)| (char, index + 1))
        .collect();

    let part1 = input
        .lines()
        .map(|line| {
            let length = line.len();
            let s1 = &line[..(length / 2)];
            let s2 = &line[(length / 2)..];

            let char = s1
                .chars()
                .find(|&c| s2.contains(c))
                .expect("Invalid input, no valid answer");
            priorities_by_char.get(&char).expect("Invalid char")
        })
        .sum::<usize>();

    let lines = input.lines().collect::<Vec<&str>>();
    let part2 = lines
        .chunks(3)
        .map(|s| {
            let s1 = s[0];
            let s2 = s[1];
            let s3 = s[2];

            let char = s1
                .chars()
                .find(|&c| s2.contains(c) && s3.contains(c))
                .expect("Invalid input, no valid answer");
            priorities_by_char.get(&char).expect("Invalid char")
        })
        .sum::<usize>();

    println!("# Day 3: Rucksack Reorganization");
    println!("Part 1 = {part1}");
    println!("Part 2 = {part2}");
}
