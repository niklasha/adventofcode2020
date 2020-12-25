use crate::day::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day21 {}

impl Day for Day21 {
    fn tag(&self) -> &str { "21" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

type Facts = HashMap<String, (Option<String>, HashSet<String>)>;

impl Day21 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = &mut io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("^((.+) )*(.*) \\(contains ((.+), )*(.*)\\)$").unwrap();
        }
        let foods = lines.map(|l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            let ingredients = format!("{}{}", cap[1].to_string(), cap[3].to_string());
            let ingredients = ingredients.split(" ").map(|s| s.to_string()).collect::<HashSet<_>>();
            let allergens = format!("{}{}", cap.get(4).map_or("", |m| m.as_str()).to_string(), cap[6].to_string());
            let allergens = allergens.split(", ").map(|s| s.to_string()).collect::<HashSet<_>>();
            (ingredients, allergens)
        }).collect_vec();
        let all_ingredients = foods.iter().flat_map(|(ingredients, _)| ingredients).map(|s| s.to_string()).collect::<HashSet<_>>();
        let all_allergens = foods.iter().flat_map(|(_, allergens)| allergens).map(|s| s.to_string()).collect::<HashSet<_>>();
        let all_allergens = all_allergens.iter().map(|allergen| (allergen.to_string(), all_ingredients.clone())).collect::<HashMap<_, _>>();
        let all_allergens = foods.iter().fold(all_allergens, |all_allergens, (ingredients, allergens)|
            all_allergens.iter().map(|(allergen, possible_ingredients)|
                (allergen.to_string(),
                 if allergens.contains(allergen) {
                     possible_ingredients.intersection(ingredients).map(|s| s.to_string()).collect::<HashSet<_>>()
                 } else { possible_ingredients.clone() }))
                .collect::<HashMap<_, _>>());
        Ok(all_ingredients.iter().filter(|&ingredient| all_allergens.values().all(|ingredients| !ingredients.contains(ingredient)))
            .map(|ingredient| foods.iter().filter(|&(ingredients, _)| ingredients.contains(ingredient)).count()).sum())
    }

    // This code is a solution, but it contains a combinatorial explosion, and likely won't terminate anytime soon.
    #[allow(dead_code)]
    fn naive_part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<usize> {
        let lines = &mut io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("^((.+) )*(.*) \\(contains ((.+), )*(.*)\\)$").unwrap();
        }
        let mut foods = lines.map(|l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            let ingredients = format!("{}{}", cap[1].to_string(), cap[3].to_string());
            let ingredients = ingredients.split(" ").map(|s| s.to_string()).collect::<HashSet<_>>();
            let allergens = format!("{}{}", cap.get(4).map_or("", |m| m.as_str()).to_string(), cap[6].to_string());
            let allergens = allergens.split(", ").map(|s| s.to_string()).collect::<HashSet<_>>();
            (ingredients, allergens)
        }).collect_vec();
        foods.sort_by_key(|(ingredients, _)| ingredients.len());

        let mut i = 0;
        let facts_base = foods.iter().fold(vec![HashMap::new()], |facts_base, (ingredients, allergens)| {
            let isp = ingredients.iter().permutations(allergens.len());
            let iasp = isp.map(|is|
                is.iter().map(|s| s.to_string()).zip(allergens.iter().map(|s| s.to_string())).collect::<HashMap<_, _>>());
            let new_facts_base = iasp.map(|ias| ingredients.iter().map(move |i| {
                    let allergen = ias.get(i).map(|s| s.to_string());
                    (i.to_string(), (allergen.clone(), allergens.iter().map(|s| s.to_string()).filter(|a| allergen != Some(a.to_string())).collect::<HashSet<_>>()))
                }).collect::<HashMap<_, _>>()).collect_vec();

            // Two facts sets are compatible if they do not contain ingredients which in one facts set contain one allergen and, in the other set, another,
            // also allergens ruled out for one ingredient in one set, must of course not be a contained allergen for the same ingredient, in the other set.
            // Another incompatibility case is that the same allergen may not be contained in more than one ingredient.
            fn is_compatible(a: &Facts, b: &Facts) -> bool {
                a.iter().all(|(id, (a_allergen, a_not_allergens))|
                    b.get(id).map_or(true, |(b_allergen, b_not_allergens)|
                        a_allergen == b_allergen ||
                            (a_allergen.is_none() && !a_not_allergens.contains(b_allergen.as_ref().unwrap())) ||
                            (b_allergen.is_none() && !b_not_allergens.contains(a_allergen.as_ref().unwrap())))) &&
                    a.iter().filter(|&(_, (allergen, _))| allergen.is_some()).map(|(id, (allergen, _))| (allergen.clone().unwrap(), id)).chain(
                        b.iter().filter(|&(_, (allergen, _))| allergen.is_some()).map(|(id, (allergen, _))| (allergen.clone().unwrap(), id)))
                        .unique().into_group_map().values().filter(|&v| v.len() >= 2).count() == 0
            }

            fn merge(a: &Facts, b: &Facts) -> Facts {
                let keys: HashSet<String> = a.keys().map(String::to_string).collect::<HashSet<_>>()
                    .union(&b.keys().map(String::to_string).collect()).map(String::to_string).collect();
                let rv = keys.iter().map(|k| (k.to_string(), {
                    let a = a.get(k).map(|a| a.clone());
                    let b = b.get(k).map(|b| b.clone());
                    let b_clone = b.clone();
                    let merged = a.map_or_else(|| b_clone.unwrap(), |a| {
                        let (a_allergen, a_not_allergens) = a.clone();
                        b.map_or(a, |(b_allergen, b_not_allergens)|
                            (a_allergen.or(b_allergen), a_not_allergens.union(&b_not_allergens).map(String::to_string).collect()))
                    });
                    merged
                })).collect();
                rv
            }

            new_facts_base.iter().cartesian_product(facts_base.iter())
                .filter(|&(new_facts, facts)| is_compatible(new_facts, facts))
                .map(|(new_facts, facts)| merge(new_facts, facts)).collect_vec()
        });
        let harmless = facts_base[0].iter().filter(|(_, (allergen, _))| allergen.is_none()).map(|(id, _)| id).collect::<HashSet<_>>();
        Ok(foods.iter().map(|(ingredients, _)| ingredients.iter().filter(|&ingredient| harmless.contains(ingredient)).count()).sum())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<String> {
        let lines = &mut io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("^((.+) )*(.*) \\(contains ((.+), )*(.*)\\)$").unwrap();
        }
        let foods = lines.map(|l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            let ingredients = format!("{}{}", cap[1].to_string(), cap[3].to_string());
            let ingredients = ingredients.split(" ").map(|s| s.to_string()).collect::<HashSet<_>>();
            let allergens = format!("{}{}", cap.get(4).map_or("", |m| m.as_str()).to_string(), cap[6].to_string());
            let allergens = allergens.split(", ").map(|s| s.to_string()).collect::<HashSet<_>>();
            (ingredients, allergens)
        }).collect_vec();
        let all_ingredients = foods.iter().flat_map(|(ingredients, _)| ingredients).map(|s| s.to_string()).collect::<HashSet<_>>();
        let all_allergens = foods.iter().flat_map(|(_, allergens)| allergens).map(|s| s.to_string()).collect::<HashSet<_>>();
        let all_allergens = all_allergens.iter().map(|allergen| (allergen.to_string(), all_ingredients.clone())).collect::<HashMap<_, _>>();
        let all_allergens = foods.iter().fold(all_allergens, |all_allergens, (ingredients, allergens)|
            all_allergens.iter().map(|(allergen, possible_ingredients)|
                (allergen.to_string(),
                 if allergens.contains(allergen) {
                     possible_ingredients.intersection(ingredients).map(|s| s.to_string()).collect::<HashSet<_>>()
                 } else { possible_ingredients.clone() }))
                .collect::<HashMap<_, _>>());
        let mut dangerous: Vec<(String, String)> = vec![];
        let mut allergens = all_allergens.clone();
        while allergens.len() > 0 {
            let (allergen, ingredients) = allergens.iter().find(|&(_, ingredients)| ingredients.len() == 1).unwrap();
            let (allergen, ingredients) = (allergen.to_string(), ingredients.clone());;
            allergens.remove(&allergen);
            let ingredient = ingredients.iter().next().unwrap().to_string();
            allergens.iter_mut().for_each(|(_, ingredients)| { ingredients.remove(&ingredient); });
            dangerous.push((allergen.to_string(), ingredient));
        }
        dangerous.sort_by_key(|(allergen, _)| allergen.clone());
        Ok(format!("{}", dangerous.iter().map(|(_, ingredient)| ingredient).join(",")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: usize) {
        assert_eq!(Day21 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)", 5);
    }

    fn test2(s: &str, f: &str) {
        assert_eq!(Day21 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f.to_string()));
    }

    #[test]
    fn part2() {
        test2("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)", "mxmxvkd,sqjhc,fvjkl");
    }
}