use regex::Regex;

const DAY_12: &str = include_str!("resources/12a.txt");

fn str_to_world(state: &str) -> WorldState {
    let moons: Vec<MoonData> = state.trim().lines().map(str_to_moon).collect();
    WorldState {
        moons: [moons[0], moons[1], moons[2], moons[3]],
    }
}

fn str_to_moon(moon_state: &str) -> MoonData {
    let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();

    for cap in re.captures_iter(moon_state.trim()) {
        let pos = [
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        ];

        let vel = [0, 0, 0];

        return MoonData { pos, vel };
    }

    panic!("No matches for moon string {}", moon_state);
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct WorldState {
    moons: [MoonData; 4],
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

fn sign_arr(vals: Vec3) -> Vec3 {
    [sign(vals[0]), sign(vals[1]), sign(vals[2])]
}

fn diff(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn plus(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn abs_total(v: Vec3) -> i64 {
    v[0].abs() + v[1].abs() + v[2].abs()
}

impl WorldState {
    fn update(&mut self) {
        let num_moons = self.moons.len();

        // update vels due to gravity
        for i in 0..num_moons {
            for j in i + 1..num_moons {
                // using copy instead of figuring out how to manage mut refs
                let mut moon_a = self.moons[i];
                let mut moon_b = self.moons[j];

                let diff_sign = sign_arr(diff(moon_a.pos, moon_b.pos));
                for i in 0..3 {
                    moon_a.vel[i] -= diff_sign[i];
                    moon_b.vel[i] += diff_sign[i];
                }

                self.moons[i] = moon_a;
                self.moons[j] = moon_b;
            }
        }

        // update pos due to vel
        for i in 0..num_moons {
            let moon = self.moons[i];
            self.moons[i].pos = plus(moon.pos, moon.vel);
        }
    }

    fn total_energy(&self) -> i64 {
        self.moons
            .iter()
            .map(|moon| moon.potential() * moon.kinetic())
            .sum()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct MoonData {
    pos: Vec3,
    vel: Vec3,
}

impl MoonData {
    fn potential(&self) -> i64 {
        abs_total(self.pos)
    }

    fn kinetic(&self) -> i64 {
        abs_total(self.vel)
    }
}

type Vec3 = [i64; 3];

pub fn a() {
    let mut world = str_to_world(DAY_12);
    const NUM_STEPS: usize = 1_000;

    for _ in 0..NUM_STEPS {
        world.update();
    }

    let total = world.total_energy();

    println!("12a: {}", total);
}

pub fn b() {
    unimplemented!()
}
