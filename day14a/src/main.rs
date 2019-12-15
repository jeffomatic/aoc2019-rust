use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_quantity(s: &str) -> (String, i64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<q>\d+) (?P<mat>\w+)").unwrap();
    }

    let caps = match RE.captures(s) {
        None => panic!("invalid quantity: {}", s),
        Some(caps) => caps,
    };

    (
        caps["mat"].parse::<String>().unwrap(),
        caps["q"].parse::<i64>().unwrap(),
    )
}

#[derive(Clone, Debug)]
struct Recipe {
    output: i64,
    ingredients: Vec<(String, i64)>,
}

fn adjust_quantity(quantities: &mut HashMap<String, i64>, k: &String, q: i64) {
    let prev = quantities.get(k).unwrap_or(&0).clone();
    quantities.insert(k.to_string(), prev + q);
}

fn get_quantity(quantities: &HashMap<String, i64>, k: &String) -> i64 {
    quantities.get(k).unwrap_or(&0).clone()
}

fn main() {
    let fuel = "FUEL".to_string();
    let ore = "ORE".to_string();

    let input = get_input();
    let mut recipes = HashMap::new();

    for line in input.lines() {
        let halves: Vec<_> = line.split(" => ").collect();
        let (material, output) = parse_quantity(halves[1]);
        let recipe = Recipe {
            output: output,
            ingredients: halves[0].split(", ").map(|s| parse_quantity(s)).collect(),
        };

        recipes.insert(material, recipe);
    }

    let mut needs = HashMap::new();
    needs.insert(fuel, 1);

    let mut stock = HashMap::new();

    loop {
        let mut next: HashMap<String, i64> = HashMap::new();

        for (mat, q) in needs.iter() {
            let have = get_quantity(&stock, mat);
            let want = max(0, *q - have);
            adjust_quantity(&mut stock, mat, *q - want);

            // There's no recipe for ore, so just move forward if we reach it as
            // a requirement.
            if *mat == ore {
                adjust_quantity(&mut next, &ore, want);
                continue;
            }

            let recipe = recipes.get(mat).unwrap();
            let mul = if recipe.output > want {
                1
            } else if want % recipe.output == 0 {
                want / recipe.output
            } else {
                (want / recipe.output) + 1
            };

            adjust_quantity(&mut stock, mat, (recipe.output * mul) - want);

            for i in recipe.ingredients.iter() {
                adjust_quantity(&mut next, &i.0, i.1 * mul);
            }
        }

        if next.len() == 1 && get_quantity(&next, &ore) > 0 {
            println!("{}", get_quantity(&next, &ore));
            return;
        }

        needs = next;
    }
}
