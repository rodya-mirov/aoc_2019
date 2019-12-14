const DAY_13: &str = include_str!("resources/13a.txt");

use std::collections::HashMap;

use crate::intcode::{str_to_ints, RunResult, VM};

pub fn a() {
    let code = str_to_ints(DAY_13);
    let mut board: HashMap<(i64, i64), i64> = HashMap::new();

    let mut vm = VM::new(&code);

    let run_result = vm.run();
    assert_eq!(run_result, RunResult::Stopped);

    while let Some(x) = vm.get_next_output() {
        let y = vm.get_next_output().expect("Y should exist");
        let tile = vm.get_next_output().expect("Tile should exist");

        board.insert((x, y), tile);
    }

    let num_blocks = board.values().filter(|tile| **tile == 2).count();

    println!("13a: {}", num_blocks);
}

pub fn b() {
    let mut code = str_to_ints(DAY_13);
    code[0] = 2;

    let mut board: HashMap<(i64, i64), i64> = HashMap::new();
    let mut score = None;

    let mut vm = VM::new(&code);

    while vm.run() == RunResult::NeedInput {
        while let Some(x) = vm.get_next_output() {
            let y = vm.get_next_output().expect("Y should exist");
            let tile = vm.get_next_output().expect("Tile should exist");

            if x == -1 {
                score = Some(tile);
            } else {
                board.insert((x, y), tile);
            }
        }

        let ball_x = board
            .iter()
            .filter(|((_x, _y), tile)| **tile == 4)
            .map(|((x, _), _)| *x)
            .next()
            .unwrap();

        let paddle_x = board
            .iter()
            .filter(|(_, tile)| **tile == 3)
            .map(|((x, _), _)| *x)
            .next()
            .unwrap();

        let change = {
            if paddle_x < ball_x {
                1
            } else if paddle_x > ball_x {
                -1
            } else {
                0
            }
        };
        vm.give_input(change);
    }

    while let Some(x) = vm.get_next_output() {
        let y = vm.get_next_output().expect("Y should exist");
        let tile = vm.get_next_output().expect("Tile should exist");

        if x == -1 {
            score = Some(tile);
        } else {
            board.insert((x, y), tile);
        }
    }

    println!("13b: {}", score.unwrap());
}
