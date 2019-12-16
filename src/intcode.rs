use std::collections::{HashMap, VecDeque};

pub fn str_to_ints(s: &str) -> Vec<i64> {
    s.trim()
        .split(',')
        .map(|token| token.trim().parse::<i64>().unwrap())
        .collect()
}

#[derive(Clone)]
struct Memory {
    start_len: usize,
    start: Vec<i64>,
    map: HashMap<usize, i64>,
}

impl Memory {
    fn new(start: Vec<i64>) -> Self {
        Memory {
            start_len: start.len(),
            start,
            map: HashMap::new(),
        }
    }

    fn get(&self, index: usize) -> i64 {
        if index < self.start_len {
            return self.start[index];
        }

        self.map.get(&index).copied().unwrap_or(0)
    }

    fn insert(&mut self, index: usize, val: i64) {
        if index < self.start_len {
            self.start[index] = val;
        }

        self.map.insert(index, val);
    }
}

#[derive(Clone)]
pub struct VM {
    code: Memory,
    ip: usize,
    relative_base: i64,
    stopped: bool,
    // available to use
    stored_inputs: VecDeque<i64>,
    // available to be polled
    stored_outputs: VecDeque<i64>,
}

impl VM {
    pub fn new(code: &[i64]) -> Self {
        let memory = Memory::new(code.into_iter().copied().collect());
        VM {
            code: memory,
            ip: 0,
            stopped: false,
            relative_base: 0,
            stored_inputs: VecDeque::new(),
            stored_outputs: VecDeque::new(),
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }

    pub fn run(&mut self) -> RunResult {
        while !self.stopped {
            // println!("Code state is now {:?}", self.code);

            let op_val = self.code.get(self.ip);
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

    fn get_val_from_memory(&self, mode: ParameterMode, ip_with_offset: usize) -> i64 {
        match mode {
            ParameterMode::Immediate => self.code.get(ip_with_offset),
            ParameterMode::Position => {
                let base_val = self.code.get(ip_with_offset);
                assert!(
                    base_val >= 0,
                    "Loaded value {} from code in positional mode, so must be nonnegative"
                );
                let actual_ind = base_val as usize;
                self.code.get(actual_ind)
            }
            ParameterMode::Relative => {
                let base_val = self.code.get(ip_with_offset) + self.relative_base;
                assert!(
                    base_val >= 0,
                    "Loaded value {} from code in relative mode, so must be nonnegative"
                );
                let actual_ind = base_val as usize;
                self.code.get(actual_ind)
            }
        }
    }

    fn set_val_in_memory(&mut self, mode: ParameterMode, ip_with_offset: usize, val: i64) {
        match mode {
            ParameterMode::Immediate => {
                panic!("Cannot set val where the index is in immediate mode")
            }
            ParameterMode::Position => {
                let dest = self.code.get(ip_with_offset);
                assert!(
                    dest >= 0,
                    "Got value {} from code in positional mode, so much be nonnegative"
                );
                let actual_ind = dest as usize;
                self.code.insert(actual_ind, val);
            }
            ParameterMode::Relative => {
                let dest = self.code.get(ip_with_offset) + self.relative_base;
                assert!(
                    dest >= 0,
                    "Got value {} from code in relative mode, so much be nonnegative"
                );
                let actual_ind = dest as usize;
                self.code.insert(actual_ind, val);
            }
        }
    }

    fn do_op(&mut self, op: Op) -> OpResult {
        let ip = self.ip;
        match op {
            Op::Add(mode_a, mode_b, mode_c) => {
                let a = self.get_val_from_memory(mode_a, ip + 1);
                let b = self.get_val_from_memory(mode_b, ip + 2);

                self.set_val_in_memory(mode_c, ip + 3, a + b);
            }
            Op::Multiply(mode_a, mode_b, mode_c) => {
                let a = self.get_val_from_memory(mode_a, ip + 1);
                let b = self.get_val_from_memory(mode_b, ip + 2);

                self.set_val_in_memory(mode_c, ip + 3, a * b);
            }
            Op::TakeInput(mode) => {
                let val = self.stored_inputs.pop_front();
                if val.is_none() {
                    return OpResult::NeedInput;
                }
                let val = val.unwrap();
                // println!("Input: {}", val);

                self.set_val_in_memory(mode, ip + 1, val);
            }
            Op::DoOutput(mode) => {
                let val = self.get_val_from_memory(mode, ip + 1);
                // println!("Output: {}", val);
                self.stored_outputs.push_back(val);
            }
            Op::JumpIfTrue(mode_a, mode_b) => {
                let a = self.get_val_from_memory(mode_a, ip + 1);
                if a != 0 {
                    // NB: we subtract two from the val because we're going to add
                    // it back at the end (it's a little janky but the alternative is to
                    // copy paste a lot of "increment self.ip" code
                    let b = self.get_val_from_memory(mode_b, ip + 2);
                    self.ip = wrapping_sub(force_usize(b), 3);
                }
            }
            Op::JumpIfFalse(mode_a, mode_b) => {
                let a = self.get_val_from_memory(mode_a, ip + 1);
                if a == 0 {
                    // NB: we subtract two from the val because we're going to add
                    // it back at the end (it's a little janky but the alternative is to
                    // copy paste a lot of "increment self.ip" code
                    let b = self.get_val_from_memory(mode_b, ip + 2);
                    self.ip = wrapping_sub(force_usize(b), 3);
                }
            }
            Op::LessThan(mode_a, mode_b, mode_c) => {
                let a = self.get_val_from_memory(mode_a, ip + 1);
                let b = self.get_val_from_memory(mode_b, ip + 2);

                let val = if a < b { 1 } else { 0 };

                self.set_val_in_memory(mode_c, ip + 3, val);
            }
            Op::Equals(mode_a, mode_b, mode_c) => {
                let a = self.get_val_from_memory(mode_a, ip + 1);
                let b = self.get_val_from_memory(mode_b, ip + 2);

                let val = if a == b { 1 } else { 0 };

                self.set_val_in_memory(mode_c, ip + 3, val);
            }
            Op::AdjustRelBase(mode) => {
                let rb_adj = self.get_val_from_memory(mode, ip + 1);
                self.relative_base += rb_adj;
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
    Relative,
}

fn to_mode(mode: usize) -> ParameterMode {
    match mode {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
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
    AdjustRelBase(ParameterMode),
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
        9 => {
            let mode_a = next_mode(&mut op_val);

            AdjustRelBase(mode_a)
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
        AdjustRelBase(_) => 2,
        Stop => 1,
    }
}
