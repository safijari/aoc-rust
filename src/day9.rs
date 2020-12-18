#[cfg(test)]
#[test]
fn test_find_summers() {
    let data = vec![35, 20, 15, 25, 47];
    assert!(has_summers(40, &data));
    assert!(!has_summers(400, &data));
}

#[test]
fn test_day9_part1() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    assert_eq!(part1(&parse_input(input), 5), 127);
}

fn parse_input(input: &str) -> Vec<i64> {
    return input.split("\n").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
}

fn has_summers(val: i64, data: &[i64]) -> bool {
    for i in 0..data.len() {
	for j in 0..data.len() {
	    if i == j {
		continue
	    }
	    if data[i] + data[j] == val {
		return true;
	    }
	}
    }

    return false;
}

fn part1(data: &Vec<i64>, preamble_len: usize) -> i64 {
    for i in preamble_len..data.len() {
	if !has_summers(data[i], &data[(i - preamble_len)..i]) {
	    return data[i];
	}
    }
    return -1;
}

fn part2(data: &Vec<i64>, sum: i64) -> i64 {
    for i in 0..data.len() {
	let mut sum_ = 0;
	for j in i..data.len() {
	    sum_ += data[j];
	    if sum_ == sum {
		if i == j {
		    break
		}
		let slice = &data[i..j];
		return slice.iter().min().unwrap() + slice.iter().max().unwrap();
	    }
	    if sum_ > sum {
		break
	    }
	}
    }

    return -1;
}

fn main() {
    let input = include_str!("./day9_input");
    let data = &parse_input(input);
    let part1_answer = part1(&data, 25);
    println!("{}, {}", part1_answer, part2(&data, part1_answer));
}
