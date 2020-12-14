use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn make_validators<'a>() -> HashMap<&'a str, Regex> {
    return vec![
        ("hgt", "(([6][0-9]|59|7[0-6])in)|((1[5-8][0-9]|19[0-3])cm)"),
        ("byr", "([1-2][9][0|2-9][0-9])|(200[0-3])"),
        ("iyr", "(201[0-9]|2020)"),
        ("eyr", "(202[0-9]|2030)"),
        ("hcl", "(#[0-9|a-f]{6})"),
        ("ecl", "(amb|blu|brn|gry|grn|hzl|oth)"),
        ("pid", "([0-9]{9})"),
    ]
    .into_iter()
    .map(|(k, v)| {
        (
            k,
            Regex::new(&format!("^{}$", v)).expect("can't initialize regex"),
        )
    })
    .collect();
}

fn main() {
    let validators = make_validators();

    let data = fs::read_to_string("/home/jari/aoc/src/day4_input").expect("can't read file");
    let mut expected_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();

    let data = data
        .split("\n\n")
        .map(|x| x.replace("\n", " "))
        .collect::<Vec<_>>();

    let mut dict_list = Vec::new();

    let mut num_valid = 0;
    let mut num_valid_strict = 0;

    for line in data.iter() {
        let mut dict = HashMap::new();
        let mut is_valid = true;
        let mut fields = HashSet::new();
        for el in line.split(" ") {
            let el = el.split(":").collect::<Vec<_>>();
            dict.insert(*el.get(0).expect(""), *el.get(1).expect(""));

            if el[0] == "cid" {
                continue;
            }

            fields.insert(el[0]);
        }
        // println!("{:?}, {:?}", fields, expected_fields);
        if fields.symmetric_difference(&expected_fields).count() == 0 {
            num_valid += 1;
            let mut valid_strict = true;
            for f_ in expected_fields.iter() {
                if !validators[f_].is_match(dict[f_]) {
                    valid_strict = false;
                }
            }
            if valid_strict {
                num_valid_strict += 1;
            }
        }

        dict_list.push(dict);
    }

    println!("{}, {} / {}", num_valid, num_valid_strict, data.len());
}
