use std::fs;

fn extract_sum(numbers: &Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    let mut out = None;
    numbers.iter().enumerate().for_each(|(i1, x1)| {
        numbers.iter().enumerate().for_each(|(i2, x2)| {
            numbers.iter().enumerate().for_each(|(i3, x3)| {
                if !(i1 == i2 && i2 == i3) && (x1 + x2 + x3 == sum) {
                    out = Some((*x1, *x2, *x3))
                }
            })
        })
    });
    out
}

fn main() {
    let extract_2020 = |numbers| extract_sum(numbers, 2020);
    let data = fs::read_to_string("/home/jari/aoc/src/day1_input").expect("can't read file");
    let numbers: Vec<i32> = data
        .split("\n")
        .map(|x| {
            x.parse()
                .expect(&format!("Could not parse {} into an int", x))
        })
        .collect();
    if let Some((n1, n2, n3)) = extract_2020(&numbers) {
        println!("{}", n1 * n2 * n3)
    }
    // extract_sum_gen(&numbers, 3, 2020);
}
