use regex::Regex;
use crate::day::*;

pub struct Day08 {}

impl Day for Day08 {
    fn tag(&self) -> &str { "08" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 5));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 0));
    }
}

impl Day08 {
    fn load<'a>(self: &Self, input: &'a mut dyn io::Read) -> Vec<(String, i64)> {
        let lines = io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("^(acc|jmp|nop) ([-+]\\d+)$").unwrap();
        }
        lines.map(|l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            (cap[1].to_string(), cap[2].parse::<i64>().unwrap())
        }).collect::<Vec<_>>()
    }

    fn process<'a>(self: &Self, p: &Vec<(String, i64)>, _i: i64) -> BoxResult<(bool, i64)> {
        let mut a = 0;
        let mut ip = 0;
        let mut v = p.iter().map(|_| false).collect::<Vec<_>>();
        while ip < v.len() && !v[ip] {
            v[ip] = true;
            let o = if p[ip].0 == "jmp" { 0 } else { 1 };
            match p[ip].0.as_str() {
                "acc" => a = a + p[ip].1,
                "jmp" => ip = (ip as i64 + p[ip].1) as usize,
                "nop" => (),
                _ => println!("unrecognized opcode")
            }
            ip = ip + o;
        }
        Ok((ip < v.len(), a))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read, i: i64) -> BoxResult<i64> {
        let p = self.load(input);
        self.process(&p, i).map(|(_, x)| x)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read, i: i64) -> BoxResult<i64> {
        let p0 = self.load(input);
        p0.iter().enumerate().filter(|(_, (o, _))| o == "nop" || o == "jmp").map(|(i, _)| i).map(|ip| {
            let mut p = p0.clone();
            p[ip] = (if p[ip].0 == "nop" { "jmp" } else { "nop" }.to_string(), p[ip].1);
            let r = self.process(&p, i);
            r
        }).find(|r| r.as_ref().ok().map(|(b, _)| b) == Some(&false)).unwrap().map(|(_, r)| r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, i: i64, f: i64) {
        assert_eq!(Day08 {}.part1_impl(&mut s.as_bytes(), i).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6", 0, 5);
    }

    fn test2(s: &str, i: i64, f: i64) {
        assert_eq!(Day08 {}.part2_impl(&mut s.as_bytes(), i).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6", 0, 8);
    }
}