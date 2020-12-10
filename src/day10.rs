use std::collections::HashMap;
use std::iter;
use crate::day::*;

pub struct Day10 {}

impl Day for Day10 {
    fn tag(&self) -> &str { "10" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day10 {
    fn numbers(self: &Self, input: &mut dyn io::Read) -> BoxResult<Vec<i64>> {
        let mut numbers = Utils::numbers(input).collect::<BoxResult<Vec<_>>>()?;
        numbers.sort();
        let last = *numbers.last().ok_or(AocError)?;
        let numbers = iter::once(0).chain(numbers).chain(iter::once(last + 3));
        Ok(numbers.collect())
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let numbers = self.numbers(input)?;
        let numbers = numbers.iter().tuple_windows().map(|(a, b)| b - a);
        let (ones, threes): (Vec<_>, Vec<_>) = numbers.partition(|&n| n == 1);
        Ok(ones.len() * threes.len())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        fn r(c: &mut HashMap<usize, usize>, v: &Vec<i64>, i: usize) -> usize {
            let rv = c.get(&i);
            if rv == None {
                let rv = if i >= v.len() - 1 { 1 } else {
                    v[i + 1..].iter().take_while(|&x| x - v[i] <= 3).enumerate().map(|(j, _)| r(c, v, i + 1 + j)).sum()
                };
                c.insert(i, rv);
                rv
            } else { *rv.unwrap() }
        }
        Ok(r(&mut HashMap::new(), &self.numbers(input)?, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day10 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("16
10
15
5
1
11
7
19
6
12
4", 7 * 5);
        test1("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3", 22 * 10);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day10 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("16
10
15
5
1
11
7
19
6
12
4", 8);
        test2("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3", 19208);
    }
}