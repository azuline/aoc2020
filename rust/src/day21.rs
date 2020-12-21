use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../../inputs/day21.txt");

type Ingredient = &'static str;
type Allergen = &'static str;
type Ingredients = HashSet<Ingredient>;
type Foods = HashMap<Allergen, Vec<Ingredients>>;

type Data = (Foods, Vec<Ingredients>);

pub fn run() {
    let data = transform_input(INPUT);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn transform_input(input: &'static str) -> Data {
    let mut foods = HashMap::new();
    let mut ingredients_list: Vec<Ingredients> = Vec::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" (contains ").unwrap();
        let ingredients: Ingredients = left.split(' ').collect();

        for allergen in right.strip_suffix(')').unwrap().split(", ") {
            foods
                .entry(allergen)
                .or_insert_with(Vec::new)
                .push(ingredients.clone());
        }

        ingredients_list.push(ingredients);
    }

    (foods, ingredients_list)
}

fn part1((foods, ingredients_list): &Data) -> usize {
    let allergen_to_ingredient = map_allergen_to_ingredient(foods);
    count_non_allergens(ingredients_list, allergen_to_ingredient)
}

fn map_allergen_to_ingredient(foods: &Foods) -> HashMap<Allergen, Vec<&Ingredient>> {
    foods
        .iter()
        .map(|(allergen, ingredients_list)| {
            // Trying to do this with fold_first fails... wasted too much time
            // trying to get that to work.
            //
            // Some issue with FromIterator not being implemented for HashSet.
            //
            // WTF?

            let first_list = &ingredients_list[0];
            let rest_of_lists = &ingredients_list[1..];

            let intersection_ingredients: Vec<&Ingredient> = first_list
                .iter()
                .filter(|&i| rest_of_lists.iter().all(|s| s.contains(i)))
                .collect();

            (*allergen, intersection_ingredients)
        })
        .collect()
}

fn count_non_allergens(
    ingredients_list: &[Ingredients],
    allergen_to_ingredient: HashMap<Allergen, Vec<&Ingredient>>,
) -> usize {
    let flagged_ingredients: HashSet<&Ingredient> =
        allergen_to_ingredient.into_values().flatten().collect();

    ingredients_list
        .iter()
        .map(|set| set.iter().collect_vec())
        .flatten()
        .filter(|i| !flagged_ingredients.contains(i))
        .count()
}

fn part2((foods, _): &Data) -> String {
    let mut allergen_to_ingredient = map_allergen_to_ingredient(foods);
    let mut canonical_dangerous_ingredient_list: Vec<(Allergen, Ingredient)> =
        Vec::new();

    while let Some((allergen, ingredients)) =
        allergen_to_ingredient.iter().find(|x| x.1.len() == 1)
    {
        let dangerous_ingredient = ingredients[0];
        canonical_dangerous_ingredient_list.push((allergen, dangerous_ingredient));

        for value in allergen_to_ingredient.values_mut() {
            value.retain(|&x| x != dangerous_ingredient);
        }
    }

    canonical_dangerous_ingredient_list
        .iter()
        .sorted_by_key(|x| x.0)
        .map(|x| x.1)
        .join(",")
}
