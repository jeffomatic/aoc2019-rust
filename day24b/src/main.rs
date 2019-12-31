use std::collections::HashSet;

type Pos = (usize, usize);

fn grid_from_string(s: &str) -> HashSet<Pos> {
    let mut g = HashSet::new();
    for (i, line) in s.trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            if c == '#' {
                g.insert((i, j));
            }
        }
    }
    g
}

fn neighbors(pos: Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    let i = pos.0;
    let j = pos.1;

    if 0 < i {
        neighbors.push((i - 1, j));
    }

    if i < 5 - 1 {
        neighbors.push((i + 1, j));
    }

    if 0 < j {
        neighbors.push((i, j - 1));
    }

    if j < 5 - 1 {
        neighbors.push((i, j + 1));
    }

    neighbors
}

fn count_adjacent(g: &HashSet<Pos>, p: Pos) -> usize {
    neighbors(p)
        .iter()
        .fold(0, |acc, n| if g.contains(n) { acc + 1 } else { acc })
}

fn working_set(g: &HashSet<Pos>) -> HashSet<Pos> {
    let mut ws = g.clone();
    for p in g.iter() {
        for n in neighbors(*p).iter() {
            ws.insert(*n);
        }
    }
    ws
}

fn simulate(g: &HashSet<Pos>) -> HashSet<Pos> {
    let mut next = HashSet::new();

    for p in working_set(g).iter() {
        let adjacent = count_adjacent(g, *p);
        if g.contains(p) {
            if adjacent == 1 {
                next.insert(*p);
            }
        } else {
            if adjacent == 1 || adjacent == 2 {
                next.insert(*p);
            }
        }
    }

    next
}

fn biodiversity_rating(g: &HashSet<Pos>) -> u64 {
    let mut rating = 0;
    for (i, j) in g.iter() {
        rating += 1 << (5 * *i) + *j;
    }
    rating
}

fn grid_to_string(g: &HashSet<Pos>) -> String {
    (0..5)
        .map(|i| {
            (0..5)
                .map(|j| if g.contains(&(i, j)) { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    let input = "
        ##.#.
        .#.##
        .#...
        #..#.
        .##..
    ";

    let mut g = grid_from_string(input);
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&grid_to_string(&g)) {
            println!(
                "{}\nrating: {}",
                grid_to_string(&g),
                biodiversity_rating(&g)
            );
            return;
        }

        seen.insert(grid_to_string(&g));
        g = simulate(&g);
    }
}
