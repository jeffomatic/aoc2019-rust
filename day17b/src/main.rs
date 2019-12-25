use intcode;
use std::io::{self, Read};

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
}

#[derive(Clone, Copy, Debug)]
struct BotState {
    pos: (i64, i64),
    orientation: Dir,
}

// returns a tuple of:
// - a 2D vector of bools, where true means the location of a scaffold
// - the current state of the bot
fn parse_map(data: &Vec<i64>) -> (Vec<Vec<bool>>, BotState) {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut bot: Option<BotState> = None;
    let mut i = 0;
    let mut j = 0;

    for c in data.iter() {
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

fn path_commands(map: &Vec<Vec<bool>>, bot: BotState) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();
    let mut bot = bot.clone();
    let mut steps = 0;

    loop {
        // Scan forward
        let p = bot.orientation.move_from(bot.pos);
        if on_scaffold(map, p) {
            bot.pos = p;
            steps += 1;
            continue;
        }

        if steps > 0 {
            path.push(steps.to_string());
            steps = 0;
        }

        let left_dir = bot.orientation.left();
        let right_dir = bot.orientation.right();

        enum Turn {
            Left,
            Right,
        }
        let mut turn: Option<Turn> = None;

        // Scan left and right
        if on_scaffold(map, left_dir.move_from(bot.pos)) {
            turn = Some(Turn::Left);
        } else if on_scaffold(map, right_dir.move_from(bot.pos)) {
            turn = Some(Turn::Right);
        }

        match turn {
            Some(turn) => match turn {
                Turn::Left => {
                    bot.orientation = left_dir;
                    path.push("L".to_string());
                }
                Turn::Right => {
                    bot.orientation = right_dir;
                    path.push("R".to_string());
                }
            },
            None => return path,
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let result = intcode::Computer::new(&program).run(&Vec::new());
    let (map, bot) = parse_map(&result.output);
    let path = path_commands(&map, bot);
    println!("{}", path.join(","));

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
