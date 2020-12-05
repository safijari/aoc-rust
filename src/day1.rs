#[macro_use]
extern crate fstrings;
use std::fs;

fn extract_sum(numbers: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    for i in 0..numbers.len() {
	for j in 0..numbers.len() {
	    if i == j {
		continue
	    }
	    if numbers[i] + numbers[j] == sum {
		return Some((numbers[i], numbers[j]))
	    }
	}
    }
    None
}


fn main() {
    let extract_2020 = |numbers| extract_sum(numbers, 2020);
    let data = fs::read_to_string("/home/jari/aoc/src/day1_input").expect("can't read file");
    let numbers: Vec<i32> = data
        .split("\n")
        .map(|x| x.parse().expect(&f!("Could not parse {x} into an int")))
        .collect();
    if let Some((n1, n2)) = extract_2020(&numbers) {
	println!("{}", n1 * n2)
    }

}
