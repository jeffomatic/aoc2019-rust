use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn ancestors<'a>(k: &str, parents: &HashMap<&str, &'a str>) -> Vec<&'a str> {
    match parents.get(k) {
        None => Vec::new(),
        Some(p) => {
            let mut res = ancestors(p, parents).to_owned();
            res.push(p);
            res
        }
    }
}

fn main() {
    let input = get_input();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    let mut all_children = HashSet::new();

    for line in input.lines() {
        let toks: Vec<&str> = line.split(")").collect();
        let parent = toks[0];
        let child = toks[1];
        all_children.insert(child);
        parents.insert(child, parent);
    }

    // find nearest common ancestor
    let you_ancestors = ancestors("YOU", &parents);
    let san_ancestors = ancestors("SAN", &parents);
    let mut nearest = 0;
    loop {
        if you_ancestors[nearest + 1] != san_ancestors[nearest + 1] {
            break;
        }
        nearest += 1;
    }

    println!(
        "{}",
        (you_ancestors.len() - nearest - 1) // de-orbits to common ancestor
        + (san_ancestors.len() - nearest - 1) // do-orbits to target
    );
}
