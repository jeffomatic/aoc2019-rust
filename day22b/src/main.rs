use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Command {
    Cut(i64),
    DealNew,
    DealWithIncrement(usize),
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<command>[A-Za-z ]+)(?P<arg>-?\d+)?").unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from("invalid command format")),
            Some(caps) => caps,
        };

        match caps["command"].parse::<String>().unwrap().as_str() {
            "cut " => Ok(Command::Cut(caps["arg"].parse::<i64>().unwrap())),
            "deal into new stack" => Ok(Command::DealNew),
            "deal with increment " => Ok(Command::DealWithIncrement(
                caps["arg"].parse::<usize>().unwrap(),
            )),
            _ => return Err(From::from("unrecognized command")),
        }
    }
}

#[derive(Debug)]
struct Deck {
    q: Vec<usize>,
}

impl Deck {
    fn new(size: usize) -> Deck {
        Deck {
            q: (0..size).collect(),
        }
    }

    fn cut(&mut self, amount: i64) {
        let len = self.q.len();
        let prev = self.q.to_vec();
        let shift = (len as i64) - amount;
        for (p, v) in prev.iter().enumerate() {
            self.q[(p + shift as usize) % len] = *v;
        }
    }

    // this is equivalent to reversing the deck
    fn deal_new(&mut self) {
        let len = self.q.len();
        let prev = self.q.to_vec();
        for (p, v) in prev.iter().enumerate() {
            self.q[len - 1 - p] = *v;
        }
    }

    fn deal_with_increment(&mut self, increment: usize) {
        let len = self.q.len();
        let prev = self.q.to_vec();
        for (p, v) in prev.iter().enumerate() {
            self.q[(p * increment) % len] = *v;
        }
    }

    fn run(&mut self, commands: &Vec<Command>) {
        for c in commands {
            match c {
                Command::Cut(n) => self.cut(*n),
                Command::DealNew => self.deal_new(),
                Command::DealWithIncrement(n) => self.deal_with_increment(*n),
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let commands: Vec<Command> = input
        .trim()
        .to_string()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut deck = Deck::new(10007);
    deck.run(&commands);
    println!(
        "{:?}",
        deck.q.iter().enumerate().find(|(_p, v)| **v == 2019)
    );
}
