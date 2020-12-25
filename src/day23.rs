use crate::day::*;
use std::collections::HashMap;

pub struct Day23 {}

impl Day for Day23 {
    fn tag(&self) -> &str { "23" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 100));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 10000000));
    }
}

impl Day23 {
    fn part1_impl(&self, input: &mut dyn io::Read, n: usize) -> BoxResult<String> {
        let mut cups = ListTable::from(
            io::BufReader::new(input).lines().next().unwrap().unwrap().chars().map(|c| c.to_digit(10).unwrap() as u32).collect_vec(), None);
        for _ in 0..n { cups.mv() }
        Ok(cups.result(8).iter().map(|&d| (d as u8 + b'0') as char).collect::<String>())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read, n: usize) -> BoxResult<usize> {
        let mut cups = ListTable::from(
            io::BufReader::new(input).lines().next().unwrap().unwrap().chars().map(|c| c.to_digit(10).unwrap() as u32).collect_vec(), Some(1000000));
        for _ in 0..n { cups.mv() }
        Ok(cups.result(2).iter().map(|&d| d as usize).product())
    }
}

trait Table/*<T: num::Integer>*/ {
    fn mv(&mut self);
    fn result(&self, n: usize) -> Vec</*T*/u32>;
}

struct NaiveTable/*<T>*/ {
    cups: Vec/*Deque*/</*T*/u32>,
    l: usize,
    min: /*T*/u32,
    max: /*T*/u32,
    p: usize,
}

impl/*<T>*/ NaiveTable/*<T>*/ {
    #[allow(dead_code)]
    fn from(init: Vec</*T*/u32>, n: Option<usize>) -> Self {
        let init_l = init.len();
        let l = n.unwrap_or(init_l);
        let cups = init.iter().map(|i| *i).chain(init_l as u32 + 1..l as u32 + 1)/*.map(|&c| c).collect::<VecDeque<_>>()*/.collect_vec();
        let (min, max) = (1, l as u32);
        Self { cups, l, min, max, p: 0, }
    }
}

impl/*<T : num::Integer>*/ Table/*<T>*/ for NaiveTable/*<T>*/ {
    fn mv(&mut self) {
        let mut d = self.cups[self.p] - 1;
        if d < self.min { d = self.max }
        let mut pick = vec![];
        let mut r = self.p + 1;
        let mut np = self.p;
        for _ in 0..3 {
            if self.cups.len() == r { r = 0 }
            pick.push(self.cups.remove(r));
            if r < self.p { np -= 1 }
        }
        while pick.contains(&d) {
            d -= 1;
            if d < self.min { d = self.max }
        }
        let mut i = self.cups.iter().position(|&x| x == d).unwrap();
        if i > np { pick.reverse(); }
        pick.iter().for_each(|&cup|
            if i > np { self.cups.insert(i + 1, cup) } else if np < self.p {
                self.cups.insert(i + 1, cup);
                np += 1;
                i += 1;
            } else {
                let c = self.cups.remove(0);
                self.cups.push/*_back*/(c);
                self.cups.insert(i, cup);
            });
        self.p = (self.p + 1) % self.l
    }

    fn result(&self, n: usize) -> Vec</*T*/u32> {
        let p = self.cups.iter().position(|&c| c == 1).unwrap() + 1;
        (0..n).map(|i| self.cups[(p + i) % self.l]).collect_vec()
    }
}

struct ListTable/*<T>*/ {
    cups: dlv_list::VecList</*T*/u32>,
    min: /*T*/u32,
    max: /*T*/u32,
    p: dlv_list::Index</*T*/u32>,
    rev: HashMap</*T*/u32, dlv_list::Index</*T*/u32>>,
}

impl/*<T>*/ ListTable/*<T>*/ {
    fn from(init: Vec</*T*/u32>, n: Option<usize>) -> Self {
        let l = init.len();
        let cups = init.iter().map(|i| *i).chain(l as u32 + 1..n.unwrap_or(l) as u32 + 1)/*.map(|&c| c).collect::<VecDeque<_>>()*/.collect::<dlv_list::VecList<_>>();
        let (min, max, p, rev) =
            (1, n.unwrap_or(l) as u32, cups.indices().next().unwrap(), cups.indices().map(|i| (*cups.get(i).unwrap(), i)).collect::<HashMap<_, _>>());
        Self { cups, min, max, p, rev }
    }
}

impl/*<T : num::Integer>*/ Table/*<T>*/ for ListTable/*<T>*/ {
    fn mv(&mut self) {
        let mut d = self.cups[self.p] - 1;
        if d < self.min { d = self.max }
        let mut pick = vec![];
        let mut r = self.cups.get_next_index(self.p);
        for _ in 0..3 {
            if r == None { r = self.cups.indices().next() }
            let nr = self.cups.get_next_index(r.unwrap());
            pick.push(self.cups.remove(r.unwrap()).unwrap());
            r = nr;
        }
        while pick.contains(&d) {
            d -= 1;
            if d < self.min { d = self.max }
        }
        let mut i = *self.rev.get(&d).unwrap();
        for &cup in pick.iter() { i = self.cups.insert_after(i, cup); self.rev.insert(cup, i); }
        self.p = self.cups.get_next_index(self.p).unwrap_or(self.cups.indices().next().unwrap());
    }

    fn result(&self, n: usize) -> Vec</*T*/u32> {
        let mut p = self.cups.indices().find(|&i| self.cups[i] == 1).unwrap();
        (0..n).map(|i| {
            p = self.cups.get_next_index(p).unwrap_or(self.cups.indices().next().unwrap());
            self.cups[p]
        }).collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, n: usize, f: &str) {
        assert_eq!(Day23 {}.part1_impl(&mut s.as_bytes(), n).ok(), Some(f.to_string()));
    }

    #[test]
    fn part1() {
        test1("389125467", 10, "92658374");
        test1("389125467", 100, "67384529");
    }

    fn test2(s: &str, n: usize, f: usize) {
        assert_eq!(Day23 {}.part2_impl(&mut s.as_bytes(), n).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("389125467", 10000000, 149245887792);
    }
}