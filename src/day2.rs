use anyhow::Result;
use std::collections::HashMap;
use std::fs;

struct PasswordInfo {
    letter: char,
    min: i32,
    max: i32,
    password: String,
}

impl PasswordInfo {
    fn from_string(instr: String) -> Result<PasswordInfo> {
        let splat: Vec<&str> = instr.split("-").collect();
        assert_eq!(splat.len(), 2);
        let min: i32 = splat[0].parse()?;
        let splat: Vec<&str> = splat[1].split(" ").collect();
        assert_eq!(splat.len(), 3);
        let max: i32 = splat[0].parse()?;
        let letter: char = splat[1].chars().collect::<Vec<char>>()[0];
        Ok(PasswordInfo {
            letter,
            min,
            max,
            password: splat[2].into(),
        })
    }

    fn count_letter(&self) -> i32 {
        let mut ct = 0;
        for c in self.password.chars() {
            if c == self.letter {
                ct += 1;
            }
        }
        ct
    }

    fn check_cond2(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        return (chars[(self.min - 1) as usize] == self.letter)
            ^ (chars[(self.max - 1) as usize] == self.letter);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let instr = String::from("15-16 l: klfbblslvjclmlnqklvg");
        let val = crate::PasswordInfo::from_string(instr.into()).unwrap();
        assert_eq!(val.letter, 'l');
        assert_eq!(val.min, 15);
        assert_eq!(val.max, 16);
        assert_eq!(val.password, "klfbblslvjclmlnqklvg");

        assert_eq!(val.count_letter(), 6);

        assert!(!val.check_cond2())
    }
}

fn main() {
    let data = fs::read_to_string("/home/jari/aoc/src/day2_input").expect("can't read file");
    let pis: Vec<_> = data
        .split("\n")
        .map(|line| PasswordInfo::from_string(line.into()).expect("Could not parse input"))
        .collect();

    let count1 = pis
        .iter()
        .map(|pi| (pi.count_letter(), pi))
        .filter(|(ct, pi)| *ct <= pi.max && *ct >= pi.min)
        .collect::<Vec<_>>()
        .len();

    let count2 = pis
        .iter()
        .filter(|pi| pi.check_cond2())
        .collect::<Vec<_>>()
        .len();

    println!("{}, {}", count1, count2)
}
