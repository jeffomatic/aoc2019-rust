use std::collections::VecDeque;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

struct Computer {
    mem: Vec<i64>,
    ip: usize,
    relative_base: usize,
    state: State,
}

#[derive(Copy, Clone, Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Copy, Clone, Debug)]
enum State {
    Ready,
    BlockedOnRead,
    Halted,
}

fn param_mode_from_int(v: i64) -> ParamMode {
    match v {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        2 => ParamMode::Relative,
        _ => panic!("invalid param mode {}", v),
    }
}

fn parse_instruction(instruction: i64) -> (i64, [ParamMode; 3]) {
    (
        instruction % 100,
        [
            param_mode_from_int((instruction % 1000) / 100),
            param_mode_from_int((instruction % 10000) / 1000),
            param_mode_from_int((instruction % 100000) / 10000),
        ],
    )
}

impl Computer {
    fn new(program: &Vec<i64>) -> Computer {
        let mut mem = vec![0; 100000];
        for i in 0..program.len() {
            mem[i] = program[i];
        }

        Computer {
            mem: mem,
            ip: 0,
            relative_base: 0,
            state: State::Ready,
        }
    }

    fn param_as_val(&self, addr: usize, mode: ParamMode) -> i64 {
        let v = self.mem[addr];
        match mode {
            ParamMode::Position => self.mem[v as usize],
            ParamMode::Immediate => v,
            ParamMode::Relative => self.mem[(self.relative_base as i64 + v) as usize],
        }
    }

    fn param_as_dst(&self, addr: usize, mode: ParamMode) -> usize {
        match mode {
            ParamMode::Position => self.mem[addr] as usize,
            ParamMode::Immediate => panic!("immediate cannot be destination"),
            ParamMode::Relative => (self.relative_base as i64 + self.mem[addr]) as usize,
        }
    }

    fn run(&mut self, input: &mut VecDeque<i64>, output: &mut VecDeque<i64>) {
        match self.state {
            State::Ready => (),
            State::BlockedOnRead => {
                if input.is_empty() {
                    return;
                }
                self.state = State::Ready;
            }
            State::Halted => return,
        }

        loop {
            let instruction = self.mem[self.ip];
            let (opcode, param_modes) = parse_instruction(instruction);

            match opcode {
                // add
                1 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    let dst = self.param_as_dst(self.ip + 3, param_modes[2]);
                    self.mem[dst] = a + b;
                    self.ip += 4;
                }
                // multiply
                2 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    let dst = self.param_as_dst(self.ip + 3, param_modes[2]);
                    self.mem[dst] = a * b;
                    self.ip += 4;
                }
                // read input
                3 => {
                    // block waiting for input
                    if input.is_empty() {
                        self.state = State::BlockedOnRead;
                        return;
                    }

                    let dst = self.param_as_dst(self.ip + 1, param_modes[0]);
                    self.mem[dst] = input.pop_front().unwrap();
                    self.ip += 2;
                }
                // write output
                4 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    output.push_back(a);
                    self.ip += 2;
                }
                // jump-if-nonzero
                5 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    self.ip = if a != 0 { b as usize } else { self.ip + 3 }
                }
                // jump-if-zero
                6 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    self.ip = if a == 0 { b as usize } else { self.ip + 3 };
                }
                // less than
                7 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    let dst = self.param_as_dst(self.ip + 3, param_modes[2]);
                    self.mem[dst] = if a < b { 1 } else { 0 };
                    self.ip += 4;
                }
                // equal
                8 => {
                    let a = self.param_as_val(self.ip + 1, param_modes[0]);
                    let b = self.param_as_val(self.ip + 2, param_modes[1]);
                    let dst = self.param_as_dst(self.ip + 3, param_modes[2]);
                    self.mem[dst] = if a == b { 1 } else { 0 };
                    self.ip += 4;
                }
                // set relative base
                9 => {
                    self.relative_base = ((self.relative_base as i64)
                        + self.param_as_val(self.ip + 1, param_modes[0]))
                        as usize;
                    self.ip += 2;
                }
                // exit
                99 => {
                    self.state = State::Halted;
                    return;
                }
                // default
                _ => panic!("address {}: invalid opcode {}", self.ip, instruction),
            };
        }
    }
}

fn main() {
    let mut program: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();

    // play for free; set address 0 to 2
    program[0] = 2;

    let mut cpu = Computer::new(&program);
    let mut input: VecDeque<i64> = VecDeque::new();
    let mut output: VecDeque<i64> = VecDeque::new();

    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        cpu.run(&mut input, &mut output);

        let mut out = Vec::new();
        for v in output.iter() {
          out.push(*v);
        }

        let mut nblocks = 0;

        for d in out.chunks(3) {
            if d[0] == -1 && d[1] == 0 {
                score = d[2];
                continue;
            }

            let x = d[0] as usize;

            // 0 is an empty tile. No game object appears in this tile.
            // 1 is a wall tile. Walls are indestructible barriers.
            // 2 is a block tile. Blocks can be broken by the ball.
            // 3 is a horizontal paddle tile. The paddle is indestructible.
            // 4 is a ball tile. The ball moves diagonally and bounces off objects.
            match d[2] {
                0 => (),
                1 => (),
                2 => nblocks += 1,
                3 => paddle_x = x,
                4 => ball_x = x,
                t => panic!("invalid tile: {}", t),
            }
        }

        match cpu.state {
          State::BlockedOnRead => {
            if paddle_x < ball_x {
                input.push_back(1);
            } else if ball_x < paddle_x {
                input.push_back(-1);
            } else {
                input.push_back(0);
            }
          }
          State::Halted => break,
          s => panic!("cpu state did not run to halt or block: {:?}", s)
        }

        if nblocks == 0 {
            break;
        }
    }

    println!("{}", score);
}
