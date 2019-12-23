use intcode;
use std::io::{self, Read};

#[derive(Debug, FromPrimitive)]
enum Dir {
    N = 1,
    S,
    W,
    E,
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn generate_map(prog: &Vec<i64>) -> (HashMap<(i64, i64), Vec<(i64, i64)>>, (i64, i64)) {}

fn main() {
    let mut program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let cpu = intcode::Computer::new(program);

    let input = HashMap::new();
    let output = HashMap::new();
}
