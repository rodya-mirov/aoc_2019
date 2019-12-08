const DAY_5: &str = include_str!("resources/5a.txt");

use crate::intcode::{str_to_ints, RunResult, VM};

fn do_a() -> i64 {
    let program = str_to_ints(DAY_5);

    let mut vm = VM::new(program);
    vm.give_input(1);

    assert_eq!(vm.run(), RunResult::Stopped);

    let output = vm.get_all_outputs();

    for i in 0..output.len() - 1 {
        assert_eq!(output[i], 0);
    }

    output[output.len() - 1]
}

pub fn a() {
    println!("5a: {}", do_a());
}

fn do_b() -> i64 {
    let program = str_to_ints(DAY_5);

    let mut vm = VM::new(program);
    vm.give_input(5);

    assert_eq!(vm.run(), RunResult::Stopped);

    let output = vm.get_all_outputs();

    for i in 0..output.len() - 1 {
        assert_eq!(output[i], 0);
    }

    output[output.len() - 1]
}

pub fn b() {
    println!("5b: {}", do_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn check_5a() {
        assert_eq!(do_a(), 16489636);
    }

    #[test]
    pub fn check_5b() {
        assert_eq!(do_b(), 9386583);
    }
}
