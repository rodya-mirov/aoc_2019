use std::cmp::{max, min};
use std::collections::HashMap;

use crate::intcode::{str_to_ints, VM};

const DAY_11: &str = include_str!("resources/11a.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_delta(self) -> (i64, i64) {
        use Direction::*;

        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }

    fn turn(self, change: i64) -> Direction {
        use Direction::*;
        match change {
            0 => match self {
                Up => Left,
                Right => Up,
                Down => Right,
                Left => Down,
            },
            1 => match self {
                Up => Right,
                Left => Up,
                Down => Left,
                Right => Down,
            },
            _ => panic!("Unrecognized direction {}", change),
        }
    }
}

struct RobotState {
    x_pos: i64,
    y_pos: i64,
    facing: Direction,
}

struct WorldState {
    // (x, y) -> color; color is always 0 or 1 but lazy; 0 is black, 1 is white
    colors: HashMap<(i64, i64), i64>,
}

impl WorldState {
    fn new() -> Self {
        WorldState {
            colors: HashMap::new(),
        }
    }

    fn get_color(&self, x: i64, y: i64) -> i64 {
        self.colors.get(&(x, y)).copied().unwrap_or(0)
    }

    fn set_color(&mut self, x: i64, y: i64, color: i64) {
        assert!(
            color == 0 || color == 1,
            "Color should be valid; got {}",
            color
        );
        self.colors.insert((x, y), color);
    }
}

impl RobotState {
    fn new(x_pos: i64, y_pos: i64) -> Self {
        RobotState {
            x_pos,
            y_pos,
            facing: Direction::Up,
        }
    }
}

pub fn a() {
    let code = str_to_ints(DAY_11);
    let mut robot_vm = VM::new(&code);

    let mut world = WorldState::new();
    let mut robot = RobotState::new(0, 0);

    while !robot_vm.is_stopped() {
        let color = world.get_color(robot.x_pos, robot.y_pos);
        robot_vm.give_input(color);

        robot_vm.run();

        let new_color = robot_vm.get_next_output().unwrap();
        let turn = robot_vm.get_next_output().unwrap();

        world.set_color(robot.x_pos, robot.y_pos, new_color);

        let new_direction = robot.facing.turn(turn);
        robot.facing = new_direction;
        let (x_delta, y_delta) = new_direction.to_delta();
        robot.x_pos += x_delta;
        robot.y_pos += y_delta;
    }

    let total_painted = world.colors.len();

    println!("11a: {}", total_painted);
}

pub fn b() {
    let code = str_to_ints(DAY_11);
    let mut robot_vm = VM::new(&code);

    let mut world = WorldState::new();
    world.set_color(0, 0, 1);

    let mut robot = RobotState::new(0, 0);

    while !robot_vm.is_stopped() {
        let color = world.get_color(robot.x_pos, robot.y_pos);
        robot_vm.give_input(color);

        robot_vm.run();

        let new_color = robot_vm.get_next_output().unwrap();
        let turn = robot_vm.get_next_output().unwrap();

        world.set_color(robot.x_pos, robot.y_pos, new_color);

        let new_direction = robot.facing.turn(turn);
        robot.facing = new_direction;
        let (x_delta, y_delta) = new_direction.to_delta();
        robot.x_pos += x_delta;
        robot.y_pos += y_delta;
    }

    let mut x_min = None;
    let mut x_max = None;
    let mut y_min = None;
    let mut y_max = None;

    for (x, y) in world.colors.keys() {
        let (x, y) = (*x, *y);

        x_min = x_min.map(|old| min(old, x)).or(Some(x));
        x_max = x_max.map(|old| max(old, x)).or(Some(x));
        y_min = y_min.map(|old| min(old, y)).or(Some(y));
        y_max = y_max.map(|old| max(old, y)).or(Some(y));
    }

    let x_min = x_min.unwrap();
    let x_max = x_max.unwrap();
    let y_min = y_min.unwrap();
    let y_max = y_max.unwrap();

    println!("11b is a picture:");

    for y in y_min..=y_max {
        let mut chars = vec![' ', ' '];

        for x in x_min..=x_max {
            let color = world.get_color(x, y);
            let c = if color == 0 { ' ' } else { '#' };
            chars.push(c);
        }
        let s = chars.into_iter().collect::<String>();

        println!("{}", s);
    }

    // TODO: print the actual picture

    // println!("11b: {}", total_painted);
}
