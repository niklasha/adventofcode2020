use crate::day::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::borrow::BorrowMut;

pub struct Day16 {}

impl Day for Day16 {
    fn tag(&self) -> &str { "16" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), "departure"));
    }
}

impl Day16 {
    fn process1(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = &mut io::BufReader::new(input).lines();
        let fields = &lines.take_while(|r| r.as_ref().unwrap() != "").map(|r| {
            let s = r.unwrap();
            let tokens = &mut s.split(": ").last().unwrap().split(" or ");
            let mut ns = tokens.next().unwrap().split("-").map(|s| s.parse::<usize>().unwrap());
            let r1 = ns.next().unwrap()..ns.next().unwrap() + 1;
            let mut ns = tokens.next().unwrap().split("-").map(|s| s.parse::<usize>().unwrap());
            let r2 = ns.next().unwrap()..ns.next().unwrap() + 1;
            (r1, r2)
        }).collect::<Vec<_>>();
        let valid = |n: usize| {
            fields.iter().any(|(r1, r2)| r1.contains(&n) || r2.contains(&n))
        };
        lines.take_while(|r| r.as_ref().unwrap() != "").map(|r| r.unwrap()).collect_vec();
        Ok(lines.filter(|r| !(*r).as_ref().unwrap().contains(":"))
            .map(|r| r.unwrap().split(",").map(|n| n.parse::<usize>().unwrap()).filter(|&n| !valid(n)).sum::<usize>())
            .sum())
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process1(input)
    }

    fn process2(self: &Self, input: &mut dyn io::Read, p: &str) -> BoxResult<usize> {
        let lines = &mut io::BufReader::new(input).lines();
        let fields = &lines.take_while(|r| r.as_ref().unwrap() != "").map(|r| {
            let s = r.unwrap();
            let mut sp = s.split(": ");
            let n = sp.next().unwrap().to_string();
            let tokens = &mut sp.next().unwrap().split(" or ");
            let mut ns = tokens.next().unwrap().split("-").map(|s| s.parse::<usize>().unwrap());
            let r1 = ns.next().unwrap()..ns.next().unwrap() + 1;
            let mut ns = tokens.next().unwrap().split("-").map(|s| s.parse::<usize>().unwrap());
            let r2 = ns.next().unwrap()..ns.next().unwrap() + 1;
            (n, r1, r2)
        }).collect::<Vec<_>>();
        let valid = |n: usize| {
            fields.iter().any(|(_, r1, r2)| r1.contains(&n) || r2.contains(&n))
        };
        let my = lines.take_while(|r| r.as_ref().unwrap() != "").map(|r| r.unwrap()).filter(|s| !s.contains(":")).collect_vec();
        let my = my[0].split(",").map(|s| s.parse::<usize>().unwrap()).collect_vec();
        let tickets = lines.filter(|r| !(*r).as_ref().unwrap().contains(":"))
            .map(|r| r.unwrap().split(",").map(|n| n.parse::<usize>().unwrap()).collect_vec()).collect_vec();
        let valid = tickets.iter().filter(|&t| t.iter().all(|&n| valid(n))).map(|r| r).collect_vec();
        let f= 0;
        let poss = &mut (0..fields.len()).map(|_| fields.iter().collect::<HashSet<_>>()).collect_vec();
        for &t in valid.iter() {
            t.iter().zip(poss.iter_mut()).for_each(|(v, fset)| fset.retain(|(_, r1, r2)| r1.contains(v) || r2.contains(v)));
        }
        let mut x = poss.iter_mut().enumerate().collect_vec();
        // XXX Manually checked that my input has exactly one column that fits for each iteration
        x.sort_by_key(|(i, p)| p.len());
        let mut s = HashSet::new();
        let mut pr = HashSet::new();
        for i in 0..x.len() {
            let t = &mut x[i];
            t.1.retain(|&e| !s.contains(&e.0));
            let f = *t.1.iter().next().unwrap();
            if f.0.starts_with(p) { pr.insert(t.0); }
            s.insert(f.0.clone());
        }
        Ok(pr.iter().map(|&c| my[c]).product())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read, p: &str) -> BoxResult<usize> {
        self.process2(input, p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day16 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12", 71);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day16 {}.part2_impl(&mut s.as_bytes(), "").ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
", 12 * 11 * 13);
    }
}