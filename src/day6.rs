use std::collections::HashSet;
use std::fs::read_to_string;

#[cfg(test)]
#[test]
fn test_thing() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    assert!(part1(input) == 11)
}

fn part1(input: &str) -> i32 {
    return input
        .split("\n\n")
        .into_iter()
        .map(|lines| {
            let uniques: HashSet<char> = lines.replace("\n", "").chars().collect();
            uniques.len() as i32
        })
        .sum();
}

fn part2(input: &str) -> i32 {
    return input
        .split("\n\n")
        .into_iter()
        .map(|lines| {
            let sets = lines
                .split("\n")
                .into_iter()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            let mut first_set = sets[0].clone();
            for set in sets.iter() {
                first_set = first_set.intersection(set).copied().collect();
            }
            first_set.len() as i32
        })
        .sum();
}

fn main() {
    let data = read_to_string("/home/jari/aoc/src/day6_input").expect("");
    println!("{}", part2(&data))
}
