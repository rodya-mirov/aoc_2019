use regex::Regex;

const DAY_12: &str = include_str!("resources/12a.txt");

fn str_to_world_dims(state: &str) -> WorldStateDims {
    #[derive(Copy, Clone)]
    struct IntVec3 {
        x: i64,
        y: i64,
        z: i64,
    }

    let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
    let mut moon_positions = Vec::new();

    for line in state.trim().lines() {
        for cap in re.captures_iter(line.trim()) {
            let pos = IntVec3 {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                z: cap[3].parse().unwrap(),
            };

            moon_positions.push(pos);
            break;
        }
    }

    assert_eq!(moon_positions.len(), 4);
    let moon_positions = [
        moon_positions[0],
        moon_positions[1],
        moon_positions[2],
        moon_positions[3],
    ];

    fn make_dim<F: Fn(IntVec3) -> i64>(moons_pos: [IntVec3; 4], f: F) -> WorldStateDim {
        WorldStateDim {
            moons: [
                MoonStateDim {
                    pos: f(moons_pos[0]),
                    vel: 0,
                },
                MoonStateDim {
                    pos: f(moons_pos[1]),
                    vel: 0,
                },
                MoonStateDim {
                    pos: f(moons_pos[2]),
                    vel: 0,
                },
                MoonStateDim {
                    pos: f(moons_pos[3]),
                    vel: 0,
                },
            ],
        }
    }

    WorldStateDims {
        x: make_dim(moon_positions, |vec| vec.x),
        y: make_dim(moon_positions, |vec| vec.y),
        z: make_dim(moon_positions, |vec| vec.z),
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct WorldStateDims {
    x: WorldStateDim,
    y: WorldStateDim,
    z: WorldStateDim,
}

impl WorldStateDims {
    fn update(&mut self) {
        self.x.update();
        self.y.update();
        self.z.update();
    }

    fn total_energy(&self) -> i64 {
        let get_potential = |moon_ind: usize| -> i64 {
            self.x.moons[moon_ind].pos.abs()
                + self.y.moons[moon_ind].pos.abs()
                + self.z.moons[moon_ind].pos.abs()
        };

        let get_kinetic = |moon_ind: usize| -> i64 {
            self.x.moons[moon_ind].vel.abs()
                + self.y.moons[moon_ind].vel.abs()
                + self.z.moons[moon_ind].vel.abs()
        };

        let get_energy =
            |moon_ind: usize| -> i64 { get_potential(moon_ind) * get_kinetic(moon_ind) };

        (0..4).map(get_energy).sum()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct WorldStateDim {
    moons: [MoonStateDim; 4],
}

impl WorldStateDim {
    fn update(&mut self) {
        for i in 0..4 {
            for j in i + 1..4 {
                let pos_diff = self.moons[i].pos - self.moons[j].pos;
                let vel_delta = sign(pos_diff);
                self.moons[i].vel -= vel_delta;
                self.moons[j].vel += vel_delta;
            }
        }

        for i in 0..4 {
            self.moons[i].pos += self.moons[i].vel;
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct MoonStateDim {
    pos: i64,
    vel: i64,
}

fn sign(val: i64) -> i64 {
    if val < 0 {
        -1
    } else if val > 0 {
        1
    } else {
        0
    }
}

fn do_total_energy(setup: &str, num_steps: usize) -> i64 {
    let mut world = str_to_world_dims(setup);

    for _ in 0..num_steps {
        world.update();
    }

    world.total_energy()
}

pub fn a() {
    let total = do_total_energy(DAY_12, 1_000);

    println!("12a: {}", total);
}

pub fn b() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12a() {
        let actual = do_total_energy(DAY_12, 1_000);
        let expected = 14907;

        assert_eq!(actual, expected);
    }
}
