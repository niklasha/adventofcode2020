use crate::day::*;

pub struct Day19 {}

impl Day for Day19 {
    fn tag(&self) -> &str { "19" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day19 {
    fn f<'a>(rules: &Vec<String>, i: usize, s: &'a str) -> (bool, &'a str) {
        let r = &rules[i];
        if r.starts_with("\"") {
            let c: char = r.chars().nth(1).unwrap();
            (s.starts_with(c), s.strip_prefix(c).unwrap_or(s))
        } else if r.contains("|") {
            let mut ok1 = false;
            let mut s1 = s;
            for t in r.split(" | ") {
                let mut ok2 = true;
                let mut s2 = s;
                for r in t.split_ascii_whitespace() {
                    if ok2 {
                        let (b, rest) = Self::f(&rules, r.parse::<usize>().unwrap(), s2);
                        if !b { ok2 = false }
                        s2 = rest
                    }
                }
                if !ok1 && ok2 {
                    ok1 = true;
                    s1 = s2
                }
            }
            (ok1, s1)
        } else {
            let mut ok = true;
            let mut s = s;
            for r in r.split_ascii_whitespace() {
                if ok {
                    let (b, rest) = Self::f(&rules, r.parse::<usize>().unwrap(), s);
                    if !b { ok = false }
                    s = rest;
                }
            }
            (ok, s)
        }
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = &mut io::BufReader::new(input).lines();
        let mut rules = Vec::new();
        &lines.take_while(|r| r.as_ref().unwrap() != "").map(|r| {
            let l = &r.unwrap();
            let sp = &mut l.split(": ");
            let i = sp.next().unwrap().parse::<usize>().unwrap();
            (i, sp.next().unwrap().to_string())
        }).sorted_by_key(|&(i, _)| i).for_each(|(i, r)| rules.insert(i, r));
        Ok(lines.filter(|r| {
            let s = (*r).as_ref().unwrap();
            let (ok, rest) = Self::f(&rules, 0, &s);
            rest.is_empty() && ok
        }).count())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = &mut io::BufReader::new(input).lines();
        let mut rules = vec![String::from(""); 1024];
        &lines.take_while(|r| r.as_ref().unwrap() != "").map(|r| {
            let l = &r.unwrap();
            let sp = &mut l.split(": ");
            let i = sp.next().unwrap().parse::<usize>().unwrap();
            let r = sp.next().unwrap();
            (i, r.to_string())
        }).sorted_by_key(|&(i, _)| i).for_each(|(i, r)| rules.insert(i, r));
        Ok(lines.filter(|r| {
            let s = (*r).as_ref().unwrap();
            // XXX guessing at a high enough limit is really ugly.
            let mut x = (2..10).flat_map(|n| (1..n)
                .map(move |m| ("42 ".repeat(m).trim().to_string(), format!("{}{}", "42 ".repeat(n - m), "31 ".repeat(n - m)).trim().to_string())));
            x.any(|(r8, r11)| {
                rules[8] = r8.to_string();
                rules[11] = r11.to_string();
                let (ok, rest) = Self::f(&rules, 0, &s);
                let ok = rest.is_empty() && ok;
                ok
            })
        }).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day19 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb", 2);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day19 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", 12);
    }
}