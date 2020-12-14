use std::collections::HashSet;
use std::fs;
#[cfg(test)]
#[test]
fn test_bsp() {
    assert_eq!(resolve_bsp("FBFBBFF"), 44);
    assert_eq!(resolve_bsp("RLR"), 5);
    assert_eq!(seat_id("FBFBBFFRLR"), 357);
}

fn resolve_bsp(pattern: &str) -> i32 {
    // F means 0, B means 1
    // L means 0, R means 1
    let binary = pattern
        .chars()
        .into_iter()
        .map(|c| if c == 'F' || c == 'L' { 0 } else { 1 })
        .collect::<Vec<i32>>();
    let sum: i32 = binary
        .iter()
        .rev()
        .enumerate()
        .map(|(p, val)| (2i32.pow(p as u32)) * val)
        .sum();
    sum
}

fn seat_id(addr: &str) -> i32 {
    return resolve_bsp(&addr[..7]) * 8 + resolve_bsp(&addr[7..]);
}

fn main() {
    let data = fs::read_to_string("/home/jari/aoc/src/day5_input").expect("can't read file");
    let data = data.split("\n").collect::<Vec<_>>();

    let ids: HashSet<i32> = data.iter().map(|a| seat_id(a)).collect();

    println!("{:?}", ids.iter().max());

    let missing_ids: Vec<i32> = (0..1024)
        .map(i32::from)
        .filter(|x| !ids.contains(x) && ids.contains(&(x + 1)) && ids.contains(&(x - 1)))
        .collect();

    print!("Missing IDS {:?}", missing_ids);
}
