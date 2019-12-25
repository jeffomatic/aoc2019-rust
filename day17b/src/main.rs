use intcode;
use std::io::{self, Read};
use std::mem;

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn from_char(c: char) -> Option<Dir> {
        match c {
            '^' => Some(Dir::N),
            'v' => Some(Dir::S),
            '>' => Some(Dir::E),
            '<' => Some(Dir::W),
            _ => None,
        }
    }

    fn left(&self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
        }
    }

    fn right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }

    fn move_from(&self, p: (i64, i64)) -> (i64, i64) {
        match self {
            Dir::N => (p.0, p.1 - 1),
            Dir::S => (p.0, p.1 + 1),
            Dir::E => (p.0 + 1, p.1),
            Dir::W => (p.0 - 1, p.1),
        }
    }

    fn turn(from: Self, to: Self) -> Option<String> {
        if mem::discriminant(&from.left()) == mem::discriminant(&to) {
            return Some("L".to_string());
        }

        if mem::discriminant(&from.right()) == mem::discriminant(&to) {
            return Some("R".to_string());
        }

        None
    }
}

#[derive(Clone, Copy, Debug)]
struct BotState {
    pos: (i64, i64),
    orientation: Dir,
}

// returns a tuple of:
// - a 2D vector of bools, where true means the location of a scaffold
// - the current state of the bot
fn parse_output(output: &Vec<i64>) -> (Vec<Vec<bool>>, BotState) {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut bot: Option<BotState> = None;
    let mut i = 0;
    let mut j = 0;

    for c in output.iter() {
        let c = *c as u8 as char;

        if c == '\n' {
            if row.len() > 0 {
                rows.push(row);
                i += 1;
            }

            row = Vec::new();
            j = 0;

            continue;
        }

        row.push(c != '.');

        if let Some(dir) = Dir::from_char(c) {
            bot = Some(BotState {
                pos: (j, i),
                orientation: dir,
            });
        }

        j += 1;
    }

    (rows, bot.unwrap())
}

fn on_scaffold(map: &Vec<Vec<bool>>, p: (i64, i64)) -> bool {
    let h = map.len() as i64;
    let w = map[0].len() as i64;
    0 <= p.0 && p.0 < w && 0 <= p.1 && p.1 < h && map[p.1 as usize][p.0 as usize]
}

fn scaffold_path(map: &Vec<Vec<bool>>, bot: BotState) -> Vec<BotState> {
    let mut path = vec![bot];
    let mut bot = bot.clone();

    'next_step: loop {
        let guesses = [
            bot.orientation,
            bot.orientation.left(),
            bot.orientation.right(),
        ];

        for d in guesses.iter() {
            let p = d.move_from(bot.pos);
            if on_scaffold(&map, p) {
                bot = BotState {
                    pos: p,
                    orientation: *d,
                };
                path.push(bot);
                continue 'next_step;
            }
        }

        // If we get here, we've exhausted our guesses, and the path is complete.
        return path;
    }
}

fn path_to_commands(path: &Vec<BotState>) -> Vec<String> {
    let mut commands = Vec::new();
    let mut forward = 1;

    for i in 0..path.len() - 1 {
        match Dir::turn(path[i].orientation, path[i + 1].orientation) {
            None => forward += 1,
            Some(s) => {
                if forward > 0 && i > 0 {
                    commands.push(forward.to_string());
                    forward = 1;
                }
                commands.push(s);
            }
        }
    }

    if forward > 0 {
        commands.push(forward.to_string());
    }

    commands
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let result = intcode::Computer::new(&program).run(&Vec::new());
    let (map, bot) = parse_output(&result.output);
    let path = scaffold_path(&map, bot);
    let commands = path_to_commands(&path);
    println!("{}", commands.join(","));

    // Set program to manual control
    program[0] = 2;

    let input_str = "A,B,B,A,B,C,A,C,B,C
L,4,L,6,L,8,L,12
L,8,R,12,L,12
R,12,L,6,L,6,L,8
n
";
    let input: Vec<i64> = input_str.chars().map(|c| c as i64).collect();

    // Run program with routine
    let result = intcode::Computer::new(&program).run(&input);
    println!("{}", result.output.last().unwrap());
}
