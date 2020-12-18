use crate::day::*;
use std::collections::VecDeque;

pub struct Day18 {}

impl Day for Day18 {
    fn tag(&self) -> &str { "18" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day18 {
    fn eval1(self: &Self, expr: &str) -> BoxResult<i64> {
        let mut op = VecDeque::new();
        let mut val = VecDeque::new();
        Ok(expr.replace("(", "( ").replace(")", " )").split_ascii_whitespace().fold(0, |rv, token| {
            match token {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                    let n = token.parse::<i64>().unwrap();
                    match op.pop_back() {
                        Some("+") => Ok(rv + n),
                        Some("*") => Ok(rv * n),
                        Some("(") => { op.push_back("("); Ok(n) },
                        Some(_) => Err(AocError),
                        None => Ok(n)
                    }
                },
                "(" => { val.push_back(rv); op.push_back("("); Ok(0) },
                ")" => {
                    let n = rv;
                    op.pop_back();
                    let rv = val.pop_back().unwrap();
                    match op.pop_back() {
                        Some("+") => Ok(rv + n),
                        Some("*") => Ok(rv * n),
                        Some("(") => { op.push_back("("); Ok(n) },
                        Some(_) => Err(AocError),
                        None => Ok(n)
                    }
                },
                "+" | "*" => { op.push_back(token); Ok(rv) },
                _ => Err(AocError),
            }.unwrap()
        }))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<i64> {
        let lines = io::BufReader::new(input).lines();
        Ok(lines.map(|r| self.eval1(&r.unwrap()).unwrap()).sum())
    }

    fn eval2(self: &Self, expr: &str) -> BoxResult<i64> {
        let mut op = VecDeque::new();
        let mut val = VecDeque::new();
        let s = expr.replace("(", "( ").replace(")", " )");
        let mut n = s.split_ascii_whitespace().fold(0, |rv, token| {
            match token {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                    let n = token.parse::<i64>().unwrap();
                    match op.pop_back() {
                        Some("+") => Ok(rv + n),
                        Some("*") => { val.push_back(rv); op.push_back("*"); Ok(n) },
                        Some("(") => { op.push_back("("); Ok(n) },
                        Some(_) => Err(AocError),
                        None => Ok(n)
                    }
                },
                "(" => { val.push_back(rv); op.push_back("("); Ok(0) },
                ")" => {
                    let mut n = rv;
                    while match op.pop_back() {
                        Some("*") => { n = val.pop_back().unwrap() * n; true },
                        _ => false,
                    } {}
                    let rv = val.pop_back().unwrap();
                    match op.pop_back() {
                        Some("+") => Ok(rv + n),
                        Some("*") => { val.push_back(rv); op.push_back("*"); Ok(n) },
                        Some("(") => { op.push_back("("); Ok(n) },
                        Some(_) => Err(AocError),
                        None => Ok(n)
                    }
                },
                "+" | "*" => { op.push_back(token); Ok(rv) },
                _ => Err(AocError),
            }.unwrap()
        });
        while match op.pop_back() {
            Some("*") => { n = val.pop_back().unwrap() * n; true },
            _ => false,
        } {}
        Ok(n)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<i64> {
        let lines = io::BufReader::new(input).lines();
        Ok(lines.map(|r| self.eval2(&r.unwrap()).unwrap()).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: i64) {
        assert_eq!(Day18 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("1 + 2 * 3 + 4 * 5 + 6", 71);
        test1("1 + (2 * 3) + (4 * (5 + 6))", 51);
        test1("2 * 3 + (4 * 5)", 26);
        test1("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437);
        test1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240);
        test1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632);
    }

    fn test2(s: &str, f: i64) {
        assert_eq!(Day18 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("1 + 2 * 3 + 4 * 5 + 6", 231);
        test2("1 + (2 * 3) + (4 * (5 + 6))", 51);
        test2("2 * 3 + (4 * 5)", 46);
        test2("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445);
        test2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060);
        test2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340);
    }
}