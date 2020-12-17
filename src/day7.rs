use anyhow::Error;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::read_to_string;

#[cfg(test)]
#[test]
fn test_day7_part1() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let el_graph = parse_string_into_graph(&input);
    assert_eq!(part1(&el_graph), 4);
    assert_eq!(part2(&el_graph), 32);
}

#[test]
fn test_day7_part2() {
    let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    let el_graph = parse_string_into_graph(&input);
    assert_eq!(part2(&el_graph), 126);
}

struct Bag {
    color: String,
    children: Vec<(String, i32)>,
    parents: Vec<String>,
}

impl fmt::Debug for Bag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} -> {} -> {:?}",
            self.parents, self.color, self.children
        )
    }
}

impl Bag {
    fn new_empty(color: &str) -> Bag {
        Bag {
            color: color.into(),
            children: Vec::new(),
            parents: Vec::new(),
        }
    }
}

fn parse_string_into_graph(input: &str) -> HashMap<String, Bag> {
    let mut bags = HashMap::new();

    for line in input.split("\n") {
        let s = line.split(" contain ").collect::<Vec<_>>();
        assert!(s.len() == 2);
        let col = s[0].split(" ").collect::<Vec<_>>()[..2].join(" ");
        let rest = s[1];
        if !bags.contains_key(&col) {
            let n = Bag::new_empty(&col.clone());
            bags.insert(col.clone(), n);
        }
        if rest == "no other bags." {
            continue;
        }
        let rest: Vec<_> = rest[..rest.len() - 1].split(", ").collect();
        for item in rest.iter() {
            let name = item.split(" ").collect::<Vec<_>>();
            let num: i32 = name[0].parse().unwrap();
            let name = name[1..3].join(" ");
            if !bags.contains_key(&name) {
                let n = Bag::new_empty(&name.clone());
                bags.insert(name.clone(), n);
            }
            bags.get_mut(&col)
                .unwrap()
                .children
                .push((name.clone(), num));
            bags.get_mut(&name).unwrap().parents.push(col.clone());
        }
    }

    bags
}

fn part1(bags: &HashMap<String, Bag>) -> i32 {
    // traversal
    let mut node = bags.get("shiny gold").unwrap();
    let mut cols = HashSet::new();
    let mut parents = node.parents.clone();

    while parents.len() > 0 {
        let cur = match parents.pop() {
            Some(val) => bags.get(&val).unwrap(),
            None => continue,
        };

        cols.insert(cur.color.clone());

        for p in cur.parents.iter() {
            parents.push(p.clone());
        }
    }

    println!("{:?}", cols);

    cols.len() as i32
}

fn part2(bags: &HashMap<String, Bag>) -> i32 {
    let mut node = bags.get("shiny gold").unwrap();
    let mut ct = 0;
    let mut children = Vec::new();
    for c in node.children.iter() {
        for _ in 0..c.1 {
            children.push(c.clone());
        }
    }

    while children.len() > 0 {
        let (cur, delta) = match children.pop() {
            Some(val) => (bags.get(&val.0).unwrap(), val.1),
            None => continue,
        };

        ct += 1;

        for c in cur.children.iter() {
            for _ in 0..c.1 {
                children.push(c.clone());
            }
        }
    }

    ct
}

fn main() {
    let map = parse_string_into_graph(&read_to_string("/home/jari/aoc/src/day7_input").unwrap());
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}
