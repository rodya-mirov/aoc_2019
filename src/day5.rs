const DAY_5: &str = include_str!("resources/5a.txt");

trait VMOutput {
    fn do_output(&mut self, val: i64);
}

struct VecOutput {
    outputs: Vec<i64>,
}

impl VecOutput {
    fn new() -> Self {
        VecOutput {
            outputs: Vec::new(),
        }
    }
}

impl VMOutput for VecOutput {
    fn do_output(&mut self, val: i64) {
        self.outputs.push(val);
    }
}

trait VMInput {
    fn get_input(&mut self) -> i64;
}

struct VecInput {
    inputs: Vec<i64>,
    counter: usize,
}

impl VecInput {
    fn new(inputs: Vec<i64>) -> Self {
        VecInput { inputs, counter: 0 }
    }
}

impl VMInput for VecInput {
    fn get_input(&mut self) -> i64 {
        if self.counter < self.inputs.len() {
            let out = self.inputs[self.counter];
            self.counter += 1;
            out
        } else {
            panic!("Attempt to read off edge of the input")
        }
    }
}

struct VM {
    code: Vec<i64>,
    ip: usize,
    stopped: bool,
}

impl VM {
    pub fn new(code: Vec<i64>) -> Self {
        VM {
            code,
            ip: 0,
            stopped: false,
        }
    }

    pub fn run<I: VMInput, O: VMOutput>(&mut self, i: &mut I, o: &mut O) {
        while !self.stopped {
            let op_val = self.code[self.ip];
            let op = to_op(op_val);

            self.do_op(op, i, o);
        }
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

    fn do_op<I: VMInput, O: VMOutput>(&mut self, op: Op, i: &mut I, o: &mut O) {
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
                let val = i.get_input();

                assert_eq!(mode, ParameterMode::Position);
                let dest = force_usize(self.code[ip + 3]);
                self.code[dest] = val;
            }
            Op::DoOutput(mode) => {
                let val = self.get_val(mode, self.code[ip + 1]);
                o.do_output(val);
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

        self.ip = wrapping_add(self.ip, skip(op));
    }
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
enum ParameterMode {
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
enum Op {
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

pub fn a() {
    let program = DAY_5
        .split(',')
        // weirdly hacky way to strip whitespace?
        .map(|token| token.split_whitespace().collect::<String>())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut vm = VM::new(program);
    let mut input = VecInput::new(vec![1]);
    let mut output = VecOutput::new();

    vm.run(&mut input, &mut output);

    let output = output.outputs;

    for i in 0..output.len() - 1 {
        assert_eq!(output[i], 0);
    }

    println!("5a: {}", output[output.len() - 1]);
}

pub fn b() {
    let program = DAY_5
        .split(',')
        // weirdly hacky way to strip whitespace?
        .map(|token| token.split_whitespace().collect::<String>())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut vm = VM::new(program);
    let mut input = VecInput::new(vec![5]);
    let mut output = VecOutput::new();

    vm.run(&mut input, &mut output);

    let output = output.outputs;

    for i in 0..output.len() - 1 {
        assert_eq!(output[i], 0);
    }

    println!("5b: {}", output[output.len() - 1]);
}
