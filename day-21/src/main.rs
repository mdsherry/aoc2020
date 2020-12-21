use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let mut allergen_map = HashMap::new();
    let mut all_ingredients = Vec::new();
    for line in include_str!("input.txt").lines() {
        let (ingredients, allergens) = line.split("(").collect_tuple().unwrap();
        let ingredients: HashSet<_> = ingredients.trim().split(' ').collect();
        all_ingredients.extend(ingredients.iter().copied());
        let allergens: Vec<_> = allergens.trim_start_matches("contains ").trim_end_matches(')').split(", ").collect();
        for allergen in allergens {
            let allergen_ingredients = allergen_map.entry(allergen).or_insert(ingredients.clone());
            let common = ingredients.intersection(&allergen_ingredients).copied().collect();
            *allergen_ingredients = common;
        }
    }
    let mut singletons: Vec<_> = allergen_map.values().filter_map(|a| if a.len() == 1 { a.iter().copied().next() } else { None }).collect();
    while let Some(singleton) = singletons.pop() {
        for (&k, v) in &mut allergen_map {
            if v.len() == 1 {
                continue;
            }
            if v.remove(singleton) {
                if v.len() == 1 {
                    singletons.push(v.iter().copied().next().unwrap());
                }
            }
        }
    }
    println!("{:#?}", allergen_map);
    let allergens: HashSet<_> = allergen_map.values().flatten().collect();
    let ingr_count = all_ingredients.iter().filter(|ingr| !allergens.contains(*ingr)).count();
    println!("{}", ingr_count);
    use std::collections::BTreeMap;
    let allergen_map: BTreeMap<_, _> = allergen_map.into_iter().collect();    
    let mut allergens: Vec<_> = allergen_map.values().flatten().copied().collect();

    println!("{}", allergens.join(","));
}
