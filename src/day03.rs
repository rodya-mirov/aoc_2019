use std::collections::{HashMap, HashSet};

const DATA_3A: &str = include_str!("resources/3a.txt");

struct WireLayout(Vec<Move>);

type Position = [i32; 2];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Move {
    direction: Direction,
    distance: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn to_direction(c: char) -> Direction {
    match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'R' => Direction::Right,
        'L' => Direction::Left,
        _ => panic!("Unrecognized direction string {}", c),
    }
}

fn add_pos(start: Position, delta: Position) -> Position {
    [start[0] + delta[0], start[1] + delta[1]]
}

fn to_delta(direction: Direction) -> [i32; 2] {
    match direction {
        Direction::Up => [0, -1],
        Direction::Down => [0, 1],
        Direction::Left => [-1, 0],
        Direction::Right => [1, 0],
    }
}

fn make_move(text: &str) -> Move {
    let chars = text.chars().collect::<Vec<char>>();

    let direction = to_direction(chars[0]);
    let distance = chars[1..].iter().collect::<String>().parse().unwrap();

    Move {
        direction,
        distance,
    }
}

fn make_wire_layout(text: &str) -> WireLayout {
    WireLayout(text.split(',').map(make_move).collect())
}

fn get_wires() -> Vec<WireLayout> {
    DATA_3A
        .lines()
        .filter(|line| line.len() > 0)
        .map(make_wire_layout)
        .collect()
}

pub fn a() {
    let mut wires = get_wires();

    assert_eq!(wires.len(), 2);

    let mut visited = HashSet::new();

    let first = wires.remove(0);
    let mut position = [0, 0];
    visited.insert(position);

    for m in first.0 {
        let delta = to_delta(m.direction);
        for _ in 0..m.distance {
            position = add_pos(position, delta);
            visited.insert(position);
        }
    }

    let second = wires.remove(0);
    let mut position = [0, 0];
    let mut least_distance = None;

    for m in second.0 {
        let delta = to_delta(m.direction);
        for _ in 0..m.distance {
            position = add_pos(position, delta);
            if position != [0, 0] && visited.contains(&position) {
                let distance = position[0].abs() + position[1].abs();
                least_distance = least_distance
                    .map(|old| std::cmp::min(old, distance))
                    .or(Some(distance));
            }
        }
    }

    println!("3a: {:?}", least_distance);
}

pub fn b() {
    let mut wires = get_wires();

    assert_eq!(wires.len(), 2);

    let mut visited = HashMap::new();

    let first = wires.remove(0);
    let mut position = [0, 0];
    let mut step = 0;
    visited.insert(position, step);

    for m in first.0 {
        let delta = to_delta(m.direction);
        for _ in 0..m.distance {
            position = add_pos(position, delta);
            step += 1;
            visited.entry(position).or_insert(step);
        }
    }

    let second = wires.remove(0);
    let mut position = [0, 0];
    let mut least_distance = None;
    let mut step = 0;

    for m in second.0 {
        let delta = to_delta(m.direction);
        for _ in 0..m.distance {
            position = add_pos(position, delta);
            step += 1;
            if position != [0, 0] && visited.contains_key(&position) {
                let distance = step + visited.get(&position).unwrap();
                least_distance = least_distance
                    .map(|old| std::cmp::min(old, distance))
                    .or(Some(distance));
            }
        }
    }

    println!("3b: {:?}", least_distance);
}
