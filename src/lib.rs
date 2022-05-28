use std::collections::VecDeque;
use std::io;
use std::process::exit;
use std::str::FromStr;

use rust_decimal::prelude::Decimal;

fn get_line() -> Result<String, String> {
    let mut s = String::new();
    match io::stdin().read_line(&mut s) {
        Ok(_) => Ok(s.trim().to_owned()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

type Num = Decimal;

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Op {
    fn calc(self, v1: Num, v2: Num) -> Result<Num, String> {
        match self {
            Self::Plus => Self::add(v1, v2),
            Self::Minus => Self::subtract(v1, v2),
            Self::Mul => Self::multiply(v1, v2),
            Self::Div => Self::divide(v1, v2),
        }
    }
    fn add(v1: Num, v2: Num) -> Result<Num, String> {
        Ok(v1 + v2)
    }
    fn subtract(v1: Num, v2: Num) -> Result<Num, String> {
        Ok(v2 - v1)
    }
    fn multiply(v1: Num, v2: Num) -> Result<Num, String> {
        Ok(v1 * v2)
    }
    fn divide(v1: Num, v2: Num) -> Result<Num, String> {
        v2.checked_div(v1)
            .ok_or_else(|| "Division by zero!".to_string())
    }
}

#[derive(Debug)]
enum Val {
    Op(Op),
    Val(Num),
}

impl FromStr for Val {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Val::Op(Op::Plus)),
            "-" => Ok(Val::Op(Op::Minus)),
            "*" => Ok(Val::Op(Op::Mul)),
            "/" => Ok(Val::Op(Op::Div)),
            v => match Decimal::from_str(v) {
                Ok(n) => Ok(Val::Val(n)),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
}

fn parse(s: String) -> Result<Vec<Val>, String> {
    s.split(' ').map(|v| v.parse::<Val>()).collect()
}

struct Calc {
    pub acc: VecDeque<Num>,
}

impl Calc {
    fn new() -> Self {
        Calc {
            acc: VecDeque::new(),
        }
    }
    fn clear(&mut self) {
        println!("Clearing stack");
        self.acc.clear();
    }
    fn push(&mut self, val: Num) {
        self.acc.push_front(val);
    }
    fn do_calc(&mut self, op: Op) -> Result<(), String> {
        if self.has_enough() {
            let (a, b) = self.pop_two();
            match op.calc(a, b) {
                Ok(v) => {
                    self.push(v);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Err("Insufficient number of operands".to_owned())
        }
    }
    fn has_enough(&self) -> bool {
        self.acc.len() >= 2
    }
    fn pop_two(&mut self) -> (Num, Num) {
        let fst = self.acc.pop_front().unwrap();
        let snd = self.acc.pop_front().unwrap();
        (fst, snd)
    }
    fn print_stack(&self) {
        println!("{:?}", self.acc);
    }
    fn ingest(&mut self, line: String) {
        match parse(line) {
            Err(e) => println!("Error: {:?}", e),
            Ok(vs) => match self._ingest(vs) {
                Ok(v) => println!("{:?}", v),
                Err(e) => println!("Error: {:?}", e),
            },
        }
    }
    fn _ingest(&mut self, vs: Vec<Val>) -> Result<Num, String> {
        for v in vs.into_iter() {
            match v {
                Val::Val(n) => self.push(n),
                Val::Op(o) => self.do_calc(o)?,
            }
        }
        match self.acc.front() {
            Some(v) => Ok(*v),
            None => Err("Empty stack".to_owned()),
        }
    }
}

fn do_exit() {
    println!("Exiting...");
    exit(0);
}

const INTRO_MSG: &str = r###"
RPN Calculator
--------------
Enter command or input, press enter to execute.
Command:
    q : quit program
    c : clear stack
    p : print stack
Input:
    +
    -
    /
    *
    Number
"###;

pub fn main_loop() -> Result<(), String> {
    println!("{}", INTRO_MSG);
    let mut calc = Calc::new();
    loop {
        let line = get_line()?;
        match line.as_str() {
            "q" => do_exit(),
            "p" => calc.print_stack(),
            "c" => calc.clear(),
            _ => calc.ingest(line),
        }
    }
}
