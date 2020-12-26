use crate::*;
use std::collections::{HashMap,
                       HashSet};

type Allergen = String;
type Ingredient = String;
#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

#[derive(Debug)]
pub struct ParsedInput {
    foods: Vec<Food>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let ingredients = separated_list1(char(' '), alpha1).map(|n: Vec<&str>| n.iter().map(|n| n.to_string()).collect());
    let allergens = map(delimited(tag("(contains "), separated_list1(tag(", "), alpha1), char(')')), |n: Vec<&str>| {
        n.iter().map(|n| n.to_string()).collect()
    });
    let food = map(separated_pair(ingredients, char(' '), allergens), |(ingredients, allergens)| Food {
        ingredients,
        allergens,
    });
    let mut parsed = map(separated_list1(line_ending, food), |foods| ParsedInput { foods });
    Ok(parsed(input)?)
}

fn find_ingredient_allergens(foods: &[Food]) -> HashMap<Ingredient, Option<Allergen>> {
    let mut allergen_map: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();

    for food in foods.iter().cloned() {
        for allergen in food.allergens {
            let ingredients = allergen_map.entry(allergen.clone()).or_insert(food.ingredients.clone());
            *ingredients = ingredients.intersection(&food.ingredients).cloned().collect();
        }
    }

    let ingredients: HashSet<Ingredient> =
        foods.iter().cloned().flat_map(|Food { ingredients, .. }| ingredients).collect();
    let mut ingredients: HashMap<Ingredient, HashSet<Allergen>> = ingredients
        .iter()
        .cloned()
        .map(|ingredient| {
            (
                ingredient.clone(),
                allergen_map
                    .iter()
                    .filter(|(_, ingredients)| ingredients.contains(&ingredient))
                    .map(|(allergen, _)| allergen.clone())
                    .collect(),
            )
        })
        .collect();

    let mut old_ingredients = None;
    let mut completed: HashMap<Allergen, Ingredient> = HashMap::new();
    while old_ingredients != Some(ingredients.clone()) {
        old_ingredients = Some(ingredients.clone());
        for (ingredient, allergens) in ingredients.iter() {
            if allergens.len() == 1 {
                completed.insert(allergens.iter().cloned().next().unwrap(), ingredient.clone());
            }
        }
        for (ingredient, allergens) in ingredients.iter_mut() {
            for allergen in allergens.clone() {
                if let Some(completed_ingredient) = completed.get(&allergen) {
                    if completed_ingredient != ingredient {
                        allergens.remove(&allergen);
                    }
                }
            }
        }
    }

    ingredients.into_iter().map(|(i, a)| (i, a.into_iter().next())).collect()
}

pub type Task1 = usize;
pub type Task2 = String;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let ingredient_allergens = find_ingredient_allergens(&input.foods);
    Ok(Output {
        task1: {
            let no_allergens: HashSet<Ingredient> = ingredient_allergens
                .iter()
                .filter(|(_, allergen)| allergen.is_none())
                .map(|(ingredient, _)| ingredient.clone())
                .collect();
            input.foods.iter().map(|food| food.ingredients.iter().filter(|&n| no_allergens.contains(n)).count()).sum()
        },
        task2: {
            let mut list: Vec<(String, String)> = ingredient_allergens
                .into_iter()
                .filter_map(|(i, a)| if let Some(a) = a { Some((i, a)) } else { None })
                .collect();
            list.sort_by(|(_, a), (_, b)| a.cmp(b));
            (&list.into_iter().map(|(i, _)| i).fold(String::new(), |buf, i| format!("{},{}", buf, i))[1..]).to_string()
        },
    })
}
