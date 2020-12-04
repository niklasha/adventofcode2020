use regex::Regex;
use std::iter;
use crate::day::*;

pub struct Day04 {}

impl Day for Day04 {
    fn tag(&self) -> &str { "04" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day04 {
    fn part1_validate(p: &str) -> bool {
        p.split(" ").map(|f| f.split(":").next()).collect::<BoxResult<Vec<_>>>()
            .map(|fs| ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().all(|&f| fs.contains(&f))) == Ok(true)
    }

    #[allow(dead_code)]
    fn part1_validate_naive(p: &str) -> bool {
        let req = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        // XXX Will accept and count duplicate fields
        p.split(" ").fold(0, |c, f| c + if req.contains(&f.split(":").next().unwrap()) { 1 } else { 0 }) == req.len()
    }

    fn part2_validate(p: &str) -> bool {
        p.split(" ").fold(0, |c, f| {
            let mut fs = f.split(":");
            let k = fs.next().unwrap();
            let v = fs.next().unwrap();
            // XXX Will accept and count duplicate fields
            c + (if match k {
                "byr" => v.parse::<i32>().map(|x| x >= 1920 && x <= 2002).ok(),
                "iyr" => v.parse::<i32>().map(|x| x >= 2010 && x <= 2020).ok(),
                "eyr" => v.parse::<i32>().map(|x| x >= 2020 && x <= 2030).ok(),
                "hgt" =>
                    Regex::new("^(\\d+)(in|cm)$").ok().and_then(|re| re.captures(v).and_then(|cap| {
                        cap[1].parse::<i32>().map(|x| if &cap[2] == "cm" { x >= 150 && x <= 193 } else { x >= 59 && x <= 76 }).ok()
                    })),
                "hcl" => Regex::new("^#[0-9a-f]{6}$").map(|re| re.is_match(v)).ok(),
                "ecl" => Some(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v)),
                "pid" =>  Regex::new("^\\d{9}$").map(|re| re.is_match(v)).ok(),
                _ => None
            } == Some(true) { 1 } else { 0 })
        }) == 7
    }

    fn process<F>(self: &Self, input: &mut dyn io::Read, validate: F) -> BoxResult<usize>
        where
            F: Fn(&str) -> bool,
    {
        let passports = io::BufReader::new(input).lines().map(Result::unwrap)
            .coalesce(|x, y| if y.is_empty() { Err((x, y)) } else { Ok(if x.is_empty() { y } else { format!("{} {}", x, y) }) });
        Ok(passports.filter(|s| validate(s)).count())
    }

    #[allow(dead_code)]
    fn naive_process<F>(self: &Self, input: &mut dyn io::Read, validate: F) -> BoxResult<usize>
        where
            F: Fn(&str) -> bool,
    {
        let lines = io::BufReader::new(input).lines();
        Ok(lines.chain(iter::once(Ok(String::new()))).map(|rs| rs)
            .scan((0, String::new()), |st, rs| {
                let s = rs.unwrap();
                let (c, p) = st;
                if s.is_empty() {
                    *c = *c + 1;
                    let v = p.clone();
                    *p = String::new();
                    Some(Some(v))
                } else {
                    if p != "" { p.push_str(" ") };
                    p.push_str(&s);
                    Some(None)
                }
            }).flatten().filter(|s| validate(&s)).count())
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input, Self::part1_validate)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        self.process(input, Self::part2_validate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day04 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
", 2);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day04 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
", 0);
        test2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
", 4);
    }
}