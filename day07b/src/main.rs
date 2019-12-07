use permutohedron::Heap;
use std::cmp;
use std::collections::VecDeque;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn param_value(mem: &Vec<i64>, addr: usize, immediate: bool) -> i64 {
    let v = mem[addr];
    if immediate {
        v
    } else {
        if v < 0 {
            panic!(
                "address {} has contains negative address value: {}",
                addr, v
            )
        }
        mem[v as usize]
    }
}

struct Simulation {
    mem: Vec<i64>,
    ip: usize,
    halted: bool,
}

impl Simulation {
    fn new(prog: &Vec<i64>) -> Simulation {
        Simulation {
            mem: prog.to_owned(),
            ip: 0,
            halted: false,
        }
    }

    fn run(&mut self, input: &mut VecDeque<i64>, output: &mut VecDeque<i64>) {
        if self.halted {
            return;
        }

        loop {
            let modes_op = self.mem[self.ip];
            let immediate_params = [
                (modes_op % 1000) >= 100,
                (modes_op % 10000) >= 1000,
                modes_op >= 10000,
            ];

            match modes_op % 100 {
                // add
                1 => {
                    if immediate_params[2] {
                        panic!("address {}: invalid opcode {}", self.ip, modes_op)
                    }

                    let a = param_value(&self.mem, self.ip + 1, immediate_params[0]);
                    let b = param_value(&self.mem, self.ip + 2, immediate_params[1]);
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = a + b;
                    self.ip += 4;
                }
                // multiply
                2 => {
                    if immediate_params[2] {
                        panic!("address {}: invalid opcode {}", self.ip, modes_op)
                    }

                    let a = param_value(&self.mem, self.ip + 1, immediate_params[0]);
                    let b = param_value(&self.mem, self.ip + 2, immediate_params[1]);
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = a * b;
                    self.ip += 4;
                }
                // read input
                3 => {
                    if input.is_empty() {
                        return;
                    }

                    let dst = self.mem[self.ip + 1] as usize;
                    self.mem[dst] = input.pop_front().unwrap();
                    self.ip += 2;
                }
                // write output
                4 => {
                    let src = self.mem[self.ip + 1] as usize;
                    output.push_back(self.mem[src]);
                    self.ip += 2;
                }
                // jump-if-nonzero
                5 => {
                    let a = param_value(&self.mem, self.ip + 1, immediate_params[0]);
                    let b = param_value(&self.mem, self.ip + 2, immediate_params[1]);
                    self.ip = if a != 0 { b as usize } else { self.ip + 3 }
                }
                // jump-if-zero
                6 => {
                    let a = param_value(&self.mem, self.ip + 1, immediate_params[0]);
                    let b = param_value(&self.mem, self.ip + 2, immediate_params[1]);
                    self.ip = if a == 0 { b as usize } else { self.ip + 3 };
                }
                // less than
                7 => {
                    if immediate_params[2] {
                        panic!("address {}: invalid opcode {}", self.ip, modes_op)
                    }

                    let a = param_value(&self.mem, self.ip + 1, immediate_params[0]);
                    let b = param_value(&self.mem, self.ip + 2, immediate_params[1]);
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = if a < b { 1 } else { 0 };
                    self.ip += 4;
                }
                // equal
                8 => {
                    if immediate_params[2] {
                        panic!("address {}: invalid opcode {}", self.ip, modes_op)
                    }

                    let a = param_value(&self.mem, self.ip + 1, immediate_params[0]);
                    let b = param_value(&self.mem, self.ip + 2, immediate_params[1]);
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = if a == b { 1 } else { 0 };
                    self.ip += 4;
                }
                // exit
                99 => {
                    self.halted = true;
                    return;
                }
                // default
                _ => panic!("address {}: invalid opcode {}", self.ip, modes_op),
            };
        }
    }
}

fn simulate(prog: &Vec<i64>, phases: &Vec<i64>) -> i64 {
    // TODO: figure out how to convince the borrow checker to use arbitrary-length
    // vectors. This is ugly as hell.
    let mut sa = Simulation::new(prog);
    let mut sb = Simulation::new(prog);
    let mut sc = Simulation::new(prog);
    let mut sd = Simulation::new(prog);
    let mut se = Simulation::new(prog);

    let mut ina = VecDeque::new();
    let mut inb = VecDeque::new();
    let mut inc = VecDeque::new();
    let mut ind = VecDeque::new();
    let mut ine = VecDeque::new();

    ina.push_back(phases[0]);
    inb.push_back(phases[1]);
    inc.push_back(phases[2]);
    ind.push_back(phases[3]);
    ine.push_back(phases[4]);

    ina.push_back(0); // initial signal

    let mut last = -1;
    loop {
        if sa.halted && sb.halted && sc.halted && sd.halted && se.halted {
            return last;
        }

        sa.run(&mut ina, &mut inb);
        sb.run(&mut inb, &mut inc);
        sc.run(&mut inc, &mut ind);
        sd.run(&mut ind, &mut ine);
        se.run(&mut ine, &mut ina);

        // track values emitted by the last simulation
        if let Some(v) = ina.back() {
            last = *v;
        }
    }
}

fn main() {
    let prog: Vec<i64> = get_input().split(",").map(|s| s.parse().unwrap()).collect();
    let mut best = -1;
    let mut phases: Vec<i64> = (5..=9).collect();
    for p in Heap::new(&mut phases) {
        best = cmp::max(best, simulate(&prog, &p));
    }
    println!("{}", best)
}
