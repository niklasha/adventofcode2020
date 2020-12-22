use crate::day::*;
use std::collections::{VecDeque, HashSet};

pub struct Day22 {}

impl Day for Day22 {
    fn tag(&self) -> &str { "22" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day22 {
    fn parse_deck(&self, player: &mut dyn Iterator<Item=Result<String, io::Error>>) -> VecDeque<usize> {
        player.skip(1).map(|r| r.unwrap().parse().unwrap()).collect()
    }

    fn parse_decks(&self, input: &mut dyn io::Read) -> (VecDeque<usize>, VecDeque<usize>) {
        let lines = &mut io::BufReader::new(input).lines();
        let mut player1 = lines.take_while(|r| r.as_ref().unwrap() != "");
        let deck1 = self.parse_deck(&mut player1);
        let mut player2 = lines.take_while(|r| r.as_ref().unwrap() != "");
        let deck2 = self.parse_deck(&mut player2);
        (deck1, deck2)
    }

    fn score(deck: &VecDeque<usize>) -> usize {
        deck.iter().zip((1..deck.len() + 1).rev()).fold(0, |sum, (a, b)| sum + a * b)
    }

    fn winning_score(deck1: &VecDeque<usize>, deck2: &VecDeque<usize>) -> usize {
        Self::score(if deck1.len() == 0 { &deck2 } else { &deck1 })
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let (mut deck1, mut deck2) = self.parse_decks(input);
        while deck1.len() != 0 && deck2.len() != 0 {
            let c1 = deck1.pop_front().unwrap();
            let c2 = deck2.pop_front().unwrap();
            if c1 > c2 {
                deck1.push_back(c1);
                deck1.push_back(c2);
            } else {
                deck2.push_back(c2);
                deck2.push_back(c1);
            }
        }
        Ok(Self::winning_score(&deck1, &deck2))
    }

    fn play(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) -> usize {
        let mut seen = HashSet::new();
        let mut winner = 1;
        let mut seen_before = false;
        while !seen_before && deck1.len() != 0 && deck2.len() != 0 {
            seen.insert((deck1.clone(), deck2.clone()));
            let c1 = deck1.pop_front().unwrap();
            let c2 = deck2.pop_front().unwrap();
            winner = if deck1.len() >= c1 && deck2.len() >= c2 {
                let mut deck1 = deck1.clone();
                deck1.truncate(c1);
                let mut deck2 = deck2.clone();
                deck2.truncate(c2);
                Self::play(&mut deck1, &mut deck2)
            } else {
                if c1 > c2 { 1 } else { 2 }
            };
            if winner == 1 {
                deck1.push_back(c1);
                deck1.push_back(c2);
            } else {
                deck2.push_back(c2);
                deck2.push_back(c1);
            }
            seen_before = seen.contains(&(deck1.clone(), deck2.clone()));
        }
        if seen_before { 1 } else { winner }
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let (mut deck1, mut deck2) = self.parse_decks(input);
        Self::play(&mut deck1, &mut deck2);
        Ok(Self::winning_score(&deck1, &deck2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day22 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10", 306);
    }

    fn test2(s: &str, f: usize) {
        assert_eq!(Day22 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10", 291);
    }
}