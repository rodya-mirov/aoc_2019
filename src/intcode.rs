use std::collections::VecDeque;

pub fn str_to_ints(s: &str) -> Vec<i64> {
    s.trim()
        .split(',')
        .map(|token| token.trim().parse::<i64>().unwrap())
        .collect()
}

pub struct VM {
    code: Vec<i64>,
    ip: usize,
    stopped: bool,
    // available to use
    stored_inputs: VecDeque<i64>,
    // available to be polled
    stored_outputs: VecDeque<i64>,
}

impl VM {
    pub fn new(code: Vec<i64>) -> Self {
        VM {
            code,
            ip: 0,
            stopped: false,
            stored_inputs: VecDeque::new(),
            stored_outputs: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> RunResult {
        while !self.stopped {
            // println!("Code state is now {:?}", self.code);

            let op_val = self.code[self.ip];
            let op = to_op(op_val);

            // println!("At ip {}, got code {} which is op {:?}", self.ip, op_val, op);

            let op_result = self.do_op(op);

            match op_result {
                OpResult::Success => {
                    self.ip = wrapping_add(self.ip, skip(op));
                }
                OpResult::NeedInput => {
                    return RunResult::NeedInput;
                }
            }
        }

        RunResult::Stopped
    }

    pub fn give_input(&mut self, input: i64) {
        self.stored_inputs.push_back(input);
    }

    pub fn get_next_output(&mut self) -> Option<i64> {
        self.stored_outputs.pop_front()
    }

    pub fn get_all_outputs(&mut self) -> Vec<i64> {
        let mut out = Vec::with_capacity(self.stored_outputs.len());
        while let Some(val) = self.stored_outputs.pop_front() {
            out.push(val);
        }
        out
    }

    fn get_val(&self, mode: ParameterMode, val: i64) -> i64 {
        match mode {
            ParameterMode::Position => {
                let pos = force_usize(val);
                self.code[pos]
            }
            ParameterMode::Immediate => val,
        }
    }

    fn do_op(&mut self, op: Op) -> OpResult {
        let ip = self.ip;
        match op {
            Op::Add(mode_a, mode_b, mode_c) => {
                let a = self.get_val(mode_a, self.code[ip + 1]);
                let b = self.get_val(mode_b, self.code[ip + 2]);

                assert_eq!(mode_c, ParameterMode::Position);
                let dest = force_usize(self.code[ip + 3]);
                self.code[dest] = a + b;
            }
            Op::Multiply(mode_a, mode_b, mode_c) => {
                let a = self.get_val(mode_a, self.code[ip + 1]);
                let b = self.get_val(mode_b, self.code[ip + 2]);

                assert_eq!(mode_c, ParameterMode::Position);
                let dest = force_usize(self.code[ip + 3]);
                self.code[dest] = a * b;
            }
            Op::TakeInput(mode) => {
                let val = self.stored_inputs.pop_front();
                if val.is_none() {
                    return OpResult::NeedInput;
                }
                let val = val.unwrap();
                // println!("Input: {}", val);

                assert_eq!(mode, ParameterMode::Position);
                let dest = force_usize(self.code[ip + 1]);
                self.code[dest] = val;
            }
            Op::DoOutput(mode) => {
                let val = self.get_val(mode, self.code[ip + 1]);
                // println!("Output: {}", val);
                self.stored_outputs.push_back(val);
            }
            Op::JumpIfTrue(mode_a, mode_b) => {
                let a = self.get_val(mode_a, self.code[ip + 1]);
                if a != 0 {
                    // NB: we subtract two from the val because we're going to add
                    // it back at the end (it's a little janky but the alternative is to
                    // copy paste a lot of "increment self.ip" code
                    let b = self.get_val(mode_b, self.code[ip + 2]);
                    self.ip = wrapping_sub(force_usize(b), 3);
                }
            }
            Op::JumpIfFalse(mode_a, mode_b) => {
                let a = self.get_val(mode_a, self.code[ip + 1]);
                if a == 0 {
                    // NB: we subtract two from the val because we're going to add
                    // it back at the end (it's a little janky but the alternative is to
                    // copy paste a lot of "increment self.ip" code
                    let b = self.get_val(mode_b, self.code[ip + 2]);
                    self.ip = wrapping_sub(force_usize(b), 3);
                }
            }
            Op::LessThan(mode_a, mode_b, mode_c) => {
                let a = self.get_val(mode_a, self.code[ip + 1]);
                let b = self.get_val(mode_b, self.code[ip + 2]);

                assert_eq!(mode_c, ParameterMode::Position);
                let dest = force_usize(self.code[ip + 3]);
                self.code[dest] = if a < b { 1 } else { 0 };
            }
            Op::Equals(mode_a, mode_b, mode_c) => {
                let a = self.get_val(mode_a, self.code[ip + 1]);
                let b = self.get_val(mode_b, self.code[ip + 2]);

                assert_eq!(mode_c, ParameterMode::Position);
                let dest = force_usize(self.code[ip + 3]);
                self.code[dest] = if a == b { 1 } else { 0 };
            }
            Op::Stop => {
                self.stopped = true;
            }
        }

        OpResult::Success
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RunResult {
    Stopped,
    NeedInput,
}

enum OpResult {
    NeedInput,
    Success,
}

fn wrapping_add(a: usize, b: usize) -> usize {
    let out = std::num::Wrapping(a) + std::num::Wrapping(b);
    out.0
}

fn wrapping_sub(a: usize, b: usize) -> usize {
    let out = std::num::Wrapping(a) - std::num::Wrapping(b);
    out.0
}

fn force_usize(val: i64) -> usize {
    if val < 0 {
        panic!("Cannot parse val {} to an index", val);
    }
    val as usize
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
}

fn to_mode(mode: usize) -> ParameterMode {
    match mode {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        _ => panic!("Unrecognized mode {}", mode),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Op {
    // Mode for param ip+1, param ip+2, param ip+3
    Add(ParameterMode, ParameterMode, ParameterMode),
    // mm for rest
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    TakeInput(ParameterMode),
    DoOutput(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    Stop,
}

fn next_mode(op_val: &mut usize) -> ParameterMode {
    let rem = *op_val % 10;
    *op_val = *op_val / 10;
    to_mode(rem)
}

fn to_op(op_val: i64) -> Op {
    use Op::*;

    if op_val < 0 {
        panic!("Cannot parse a negative op! Received {}", op_val);
    }

    let mut op_val = op_val as usize;

    let simple_op = op_val % 100;
    op_val /= 100;

    match simple_op {
        1 => {
            let mode_a = next_mode(&mut op_val);
            let mode_b = next_mode(&mut op_val);
            let mode_c = next_mode(&mut op_val);

            Add(mode_a, mode_b, mode_c)
        }
        2 => {
            let mode_a = next_mode(&mut op_val);
            let mode_b = next_mode(&mut op_val);
            let mode_c = next_mode(&mut op_val);

            Multiply(mode_a, mode_b, mode_c)
        }
        3 => {
            let mode = next_mode(&mut op_val);
            TakeInput(mode)
        }
        4 => {
            let mode = next_mode(&mut op_val);
            DoOutput(mode)
        }
        5 => {
            let mode_a = next_mode(&mut op_val);
            let mode_b = next_mode(&mut op_val);

            JumpIfTrue(mode_a, mode_b)
        }
        6 => {
            let mode_a = next_mode(&mut op_val);
            let mode_b = next_mode(&mut op_val);

            JumpIfFalse(mode_a, mode_b)
        }
        7 => {
            let mode_a = next_mode(&mut op_val);
            let mode_b = next_mode(&mut op_val);
            let mode_c = next_mode(&mut op_val);

            LessThan(mode_a, mode_b, mode_c)
        }
        8 => {
            let mode_a = next_mode(&mut op_val);
            let mode_b = next_mode(&mut op_val);
            let mode_c = next_mode(&mut op_val);

            Equals(mode_a, mode_b, mode_c)
        }
        99 => Stop,
        _ => panic!("Unrecognized op code {}", simple_op),
    }
}

fn skip(op: Op) -> usize {
    use Op::*;

    match op {
        Add(_, _, _) => 4,
        Multiply(_, _, _) => 4,
        TakeInput(_) => 2,
        DoOutput(_) => 2,
        JumpIfTrue(_, _) => 3,
        JumpIfFalse(_, _) => 3,
        LessThan(_, _, _) => 4,
        Equals(_, _, _) => 4,
        Stop => 1,
    }
}
