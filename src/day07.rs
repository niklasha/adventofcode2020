use regex::Regex;
use std::collections::HashMap;
use crate::day::*;

pub struct Day07 {}

impl Day for Day07 {
    fn tag(&self) -> &str { "07" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day07 {
    fn process<'a, F>(self: &Self, input: &'a mut dyn io::Read, f: F) -> BoxResult<usize>
        where F: Fn(&HashMap<String, Vec<(usize, String)>>) -> usize {
        lazy_static! {
            static ref RE: Regex = Regex::new("^(no|\\d+) (.+) (bag.?)$").unwrap();
        }
        let bags = io::BufReader::new(input).lines().map(Result::unwrap)
            .map(|s: String| {
                let v = s.split(" contain ").collect::<Vec<_>>();
                let (o, i) = (v[0], v[1]);
                let o = o[..o.len() - 5].to_string();
                let is = i[..i.len() - 1].split(", ").map(|s| {
                    let c = RE.captures(s).unwrap();
                    let (n, k) = (c[1].to_string(), c[2].to_string());
                    let n = if &n == "no" { 0 } else { n.parse::<usize>().unwrap() };
                    (n, k)
                }).collect::<Vec<(_, String)>>();
                (o, is)
            }).collect::<HashMap<_, _>>();
        Ok(f(&bags))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input, |bags|
            bags.iter().map(|(_, v)| {
                fn contains_shiny_gold(bags: &HashMap<String, Vec<(usize, String)>>, v: &Vec<(usize, String)>) -> bool {
                    v[0].0 != 0 && v.iter().find(|(_, b)| b == "shiny gold" || contains_shiny_gold(bags, bags.get(b).unwrap())) != None
                }
                contains_shiny_gold(&bags, v)
            }).filter(|b| *b).count()
        )
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        fn count(bags: &HashMap<String, Vec<(usize, String)>>, b: &str) -> usize {
            let v =  bags.get(b).unwrap();
            1 + if v[0].0 == 0 { 0 } else { v.iter().map(|(n, b)| n * count(bags, b)).sum() }
        }
        self.process(input, |bags| bags.get("shiny gold").unwrap().iter().map(|(n, b)| n * count(&bags, b)).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day07 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
", 4);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day07 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
", 32);
        test2("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
", 126);
    }
}