#[macro_use]
extern crate fstrings;
use std::fs;

fn count_trees(data: &Vec<Vec<char>>, slope: (i32, i32)) -> i64 {
    let w = data[0].len();
    let h = data.len();
    data.iter().for_each(|x| assert_eq!(x.len(), w));

    let (sx, sy) = slope;
    let (mut x, mut y) = (0 as usize, 0 as usize);
    let mut num_trees = 0;
    loop {
	x += sx as usize;
	x = x % w;
	y += sy as usize;
	if y >= h {break};
	if data[y][x] == '#' {num_trees += 1};
    };
    num_trees
}

fn main() {
    let data = fs::read_to_string("/home/jari/aoc/src/day3_input").expect("can't read file").split("\n").map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let counts: Vec<i64> = slopes.iter().map(|x| count_trees(&data, *x)).collect();

    println!("{:?}", counts.iter().product::<i64>())
}