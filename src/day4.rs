use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

#[cfg(test)]
#[test]
fn test_validate_year() {
    let by = |y| validate_year(y, 1920, 2002);
    assert!(by("1920"));
    assert!(by("2002"));
    assert!(!by("b002"));
    assert!(!by("202"));
    assert!(!by("1919"));
    assert!(!by("2003"));
    assert!(by("1950"));
}

#[test]
fn test_validate_height() {
    let f = validate_height;
    assert!(f("150cm"));
    assert!(f("180cm"));
    assert!(f("193cm"));
    assert!(f("59in"));
    assert!(f("70in"));
    assert!(f("76in"));

    assert!(!f("50cm"));
    assert!(!f("180m"));
    assert!(!f("193cmcm"));
    assert!(!f("599in"));
    assert!(!f("70iin"));
    assert!(!f("7in"));
}

fn validate_height(ht: &str) -> bool {
    let r = Regex::new(r"^(([6][0-9]|59|7[0-6])in)|((1[5-8][0-9]|19[0-3])cm)$")
        .expect("can't make regex");
    return r.is_match(ht);
}

fn validate_year(yr: &str, min_yr: i32, max_yr: i32) -> bool {
    if yr.len() != 4 {
        return false;
    }

    let yr: i32 = match yr.parse() {
        Ok(data) => data,
        Err(err) => return false,
    };

    if !(yr >= min_yr && yr <= max_yr) {
        return false;
    }

    return true;
}

fn main() {
    let validators: HashMap<&str, _> =
        vec![("hgt", "(([6][0-9]|59|7[0-6])in)|((1[5-8][0-9]|19[0-3])cm)"),
	]
            .into_iter()
            .map(|(k, v)| (k, Regex::new(v).expect("can't initialize regex")))
            .collect();

    println!("{:?}", validators);

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
        }

        dict_list.push(dict);
    }

    // let data = data.iter().map(|s| s.split(" "));

    println!("{}, {}", num_valid, data.len());
}
