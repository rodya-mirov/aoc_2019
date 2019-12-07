const DAY_5: &str = include_str!("resources/5a.txt");

use crate::intcode::{str_to_ints, VecInput, VecOutput, VM};

pub fn a() {
    let program = str_to_ints(DAY_5);

    let mut vm = VM::new(program);
    let mut input = VecInput::new(vec![1]);
    let mut output = VecOutput::new();

    vm.run(&mut input, &mut output);

    let output = output.to_vec();

    for i in 0..output.len() - 1 {
        assert_eq!(output[i], 0);
    }

    println!("5a: {}", output[output.len() - 1]);
}

pub fn b() {
    let program = str_to_ints(DAY_5);

    let mut vm = VM::new(program);
    let mut input = VecInput::new(vec![5]);
    let mut output = VecOutput::new();

    vm.run(&mut input, &mut output);

    let output = output.to_vec();

    for i in 0..output.len() - 1 {
        assert_eq!(output[i], 0);
    }

    println!("5b: {}", output[output.len() - 1]);
}
