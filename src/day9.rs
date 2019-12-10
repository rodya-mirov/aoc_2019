const DAY_9: &str = include_str!("resources/9a.txt");

use crate::intcode::{str_to_ints, RunResult, VM};

pub fn a() {
    let code = str_to_ints(DAY_9);

    let mut vm = VM::new(&code);
    vm.give_input(1);

    let run_result = vm.run();
    assert_eq!(run_result, RunResult::Stopped);

    let outputs = vm.get_all_outputs();
    assert_eq!(outputs.len(), 1);

    println!("9a: {}", outputs[0]);
}

pub fn b() {
    let code = str_to_ints(DAY_9);

    let mut vm = VM::new(&code);
    vm.give_input(2);

    let run_result = vm.run();
    assert_eq!(run_result, RunResult::Stopped);

    let outputs = vm.get_all_outputs();
    assert_eq!(outputs.len(), 1);
    let out = outputs[0];

    println!("9b: {}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9a() {
        let code = str_to_ints(DAY_9);

        let mut vm = VM::new(&code);
        vm.give_input(1);

        let run_result = vm.run();
        assert_eq!(run_result, RunResult::Stopped);

        let outputs = vm.get_all_outputs();
        assert_eq!(outputs.len(), 1);

        assert_eq!(outputs[0], 2738720997);
    }

    #[test]
    fn test_9b() {
        let code = str_to_ints(DAY_9);

        let mut vm = VM::new(&code);
        vm.give_input(2);

        let run_result = vm.run();
        assert_eq!(run_result, RunResult::Stopped);

        let outputs = vm.get_all_outputs();
        assert_eq!(outputs.len(), 1);

        assert_eq!(outputs[0], 50894);
    }

    #[test]
    fn test_a() {
        let code = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut vm = VM::new(&code);
        vm.run();

        let actual_output = vm.get_all_outputs();

        let expected_output = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_b() {
        let code = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let mut vm = VM::new(&code);
        vm.run();

        let actual_output = vm.get_all_outputs();

        assert_eq!(actual_output.len(), 1);
        let act = actual_output[0];

        println!("Actual output: {}", act);

        assert_eq!(act.to_string().chars().count(), 16);
    }

    #[test]
    fn test_c() {
        let code = [104, 1125899906842624, 99];

        let mut vm = VM::new(&code);
        vm.run();

        let actual_output = vm.get_all_outputs();

        let expected_output = vec![1125899906842624];

        assert_eq!(actual_output, expected_output);
    }
}
