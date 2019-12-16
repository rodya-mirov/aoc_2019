const DAY_15: &str = include_str!("resources/15a.txt");

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::intcode::{str_to_ints, RunResult, VM};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn with(self, vel: Vel) -> Pos {
        Pos {
            x: self.x + vel.x,
            y: self.y + vel.y,
        }
    }

    fn with_dir(self, dir: Direction) -> Pos {
        let vel = dir.to_vel();
        self.with(vel)
    }

    fn neighbors(self) -> [Pos; 4] {
        use Direction::*;

        [
            self.with_dir(Up),
            self.with_dir(Down),
            self.with_dir(Left),
            self.with_dir(Right),
        ]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Vel {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug)]
struct World {
    known: HashMap<Pos, TileState>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl World {
    fn new() -> Self {
        let mut known = HashMap::new();
        known.insert(Pos { x: 0, y: 0 }, TileState::Open);

        World {
            known,
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }

    fn spread_time(&self, start: Pos) -> usize {
        let mut seen: HashSet<Pos> = HashSet::new();
        // TODO optimization; we don't need to enqueue a jillion usizes, we can be smarter about VecDeque trading
        let mut to_process: VecDeque<(usize, Pos)> = VecDeque::new();

        seen.insert(start);
        to_process.push_back((0, start));

        loop {
            let (path_len, next_place) = to_process.pop_front().unwrap();
            for neighbor in next_place.neighbors().iter().copied() {
                if self.known.get(&neighbor).copied().unwrap() == TileState::Wall
                    || seen.contains(&neighbor)
                {
                    continue;
                }

                to_process.push_back((path_len + 1, neighbor));
                seen.insert(neighbor);
            }

            if to_process.is_empty() {
                return path_len;
            }
        }
    }

    // Pretty straightforward BFS
    fn shortest_path_len(&self, start: Pos, end: Pos) -> usize {
        let mut seen: HashSet<Pos> = HashSet::new();
        // TODO optimization; we don't need to enqueue a jillion usizes, we can be smarter about VecDeque trading
        let mut to_process: VecDeque<(usize, Pos)> = VecDeque::new();

        seen.insert(start);
        to_process.push_back((0, start));

        loop {
            let (path_len, next_place) = to_process.pop_front().unwrap();
            for neighbor in next_place.neighbors().iter().copied() {
                if neighbor == end {
                    return path_len + 1;
                }

                if self.known.get(&neighbor).copied().unwrap() == TileState::Wall
                    || seen.contains(&neighbor)
                {
                    continue;
                }

                to_process.push_back((path_len + 1, neighbor));
                seen.insert(neighbor);
            }
        }
    }

    // Used for debug output, just leave it in
    #[allow(dead_code)]
    fn to_string(&self, robot_pos: Pos) -> String {
        // Pre: robot_pos is in self.bounds
        let mut out = String::new();

        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                let pos = Pos { x, y };
                let to_push = {
                    if robot_pos == pos {
                        'R'
                    } else {
                        match self.get_state(Pos { x, y }) {
                            None => ' ',
                            Some(TileState::Open) => '.',
                            Some(TileState::Wall) => '#',
                            Some(TileState::Oxygen) => '0',
                        }
                    }
                };

                out.push(to_push);
            }
            out.push('\n');
        }

        out
    }

    fn get_state(&self, pos: Pos) -> Option<TileState> {
        self.known.get(&pos).copied()
    }

    fn set_state(&mut self, pos: Pos, state: TileState) {
        let old = self.known.insert(pos, state);
        if let Some(old_state) = old {
            assert_eq!(old_state, state);
        } else {
            self.x_min = min(self.x_min, pos.x);
            self.x_max = max(self.x_max, pos.x);
            self.y_min = min(self.y_min, pos.y);
            self.y_max = max(self.y_max, pos.y);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum TileState {
    Open,
    Wall,
    Oxygen, // target
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_vel(self) -> Vel {
        use Direction::*;

        match self {
            Left => Vel { x: -1, y: 0 },
            Right => Vel { x: 1, y: 0 },
            Up => Vel { x: 0, y: -1 },
            Down => Vel { x: 0, y: 1 },
        }
    }

    fn to_command(self) -> i64 {
        use Direction::*;

        match self {
            Left => 3,
            Right => 4,
            Up => 1,
            Down => 2,
        }
    }

    fn invert(self) -> Direction {
        use Direction::*;

        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum RobotResponse {
    Moved,
    HitWall,
    MovedAndFoundOxygen,
}

impl RobotResponse {
    fn from_output(output: i64) -> Self {
        use RobotResponse::*;

        match output {
            0 => HitWall,
            1 => Moved,
            2 => MovedAndFoundOxygen,
            _ => panic!("Unexpected robot response {}", output),
        }
    }
}

fn build_map(code: &[i64]) -> World {
    use Direction::*;
    use RobotResponse::*;

    let mut world = World::new();

    let mut vm = VM::new(code);
    assert_eq!(vm.run(), RunResult::NeedInput);
    assert_eq!(vm.get_next_output(), None);

    // but now ... ugh?
    let mut search_from: HashMap<Pos, VM> = HashMap::new();
    search_from.insert(Pos { x: 0, y: 0 }, vm);

    // Every time we make a move, if we discover a new place, we drop a pin there
    // which is a clone of the VM we used to get there. This is a lot of clones, but
    // VMs aren't _that_ expensive, and the map is fairly small.
    //
    // If VMs were more expensive (or if we really wanted to be fair to the theme)
    // we would probably leave backtracking instructions or something but this is faster
    // and easier to code
    while !search_from.is_empty() {
        let pos = search_from.keys().next().copied().unwrap();
        let mut vm = search_from.remove(&pos).unwrap();

        for &dir in &[Up, Left, Right, Down] {
            let next_pos = pos.with_dir(dir);
            if world.get_state(next_pos).is_none() {
                vm.give_input(dir.to_command());
                assert_eq!(vm.run(), RunResult::NeedInput);

                match RobotResponse::from_output(vm.get_next_output().unwrap()) {
                    Moved => {
                        world.set_state(next_pos, TileState::Open);
                        search_from.entry(next_pos).or_insert_with(|| vm.clone());
                        vm.give_input(dir.invert().to_command());
                    }
                    MovedAndFoundOxygen => {
                        world.set_state(next_pos, TileState::Oxygen);
                        search_from.entry(next_pos).or_insert_with(|| vm.clone());
                        vm.give_input(dir.invert().to_command());
                    }
                    HitWall => {
                        world.set_state(next_pos, TileState::Wall);
                    }
                }

                assert_eq!(vm.run(), RunResult::NeedInput);
                let _ = vm.get_next_output(); // ignored, because it's backtracking
            }
        }
    }

    world
}

pub fn a() {
    let code = &str_to_ints(DAY_15);
    let map = build_map(&code);

    let start_pos = Pos { x: 0, y: 0 };
    let oxygen_pos: Pos = map
        .known
        .iter()
        .filter(|(_k, v)| **v == TileState::Oxygen)
        .map(|(k, _v)| *k)
        .next()
        .unwrap();

    let shortest_path_len = map.shortest_path_len(start_pos, oxygen_pos);

    println!("15a: {}", shortest_path_len);
}

pub fn b() {
    let code = &str_to_ints(DAY_15);
    let map = build_map(&code);

    let oxygen_pos: Pos = map
        .known
        .iter()
        .filter(|(_k, v)| **v == TileState::Oxygen)
        .map(|(k, _v)| *k)
        .next()
        .unwrap();

    let spread_time = map.spread_time(oxygen_pos);

    println!("15b: {}", spread_time);
}
