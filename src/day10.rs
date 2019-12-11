use std::cmp::{min, Ordering};
use std::collections::BTreeMap;

const DAY_10: &str = include_str!("resources/10a.txt");

struct AsteroidField {
    locs: Vec<(usize, usize)>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct LineOfSight {
    // d = h - t
    // t = h - d
    // home minus target; so x_diff < 0 means target is LEFT of home
    x_diff: i64,
    // likewise; y_diff < 0 means target is UP of home
    y_diff: i64,
}

impl LineOfSight {
    fn mag(&self) -> i64 {
        self.x_diff * self.x_diff + self.y_diff * self.y_diff
    }

    fn to_canonical(&self) -> LineOfSight {
        // this is ... real sloppy gcd, ugh
        if self.x_diff == 0 {
            return LineOfSight {
                x_diff: 0,
                y_diff: sign(self.y_diff).to_int(),
            };
        } else if self.y_diff == 0 {
            return LineOfSight {
                x_diff: sign(self.x_diff).to_int(),
                y_diff: 0,
            };
        }
        let mut out_x = self.x_diff;
        let mut out_y = self.y_diff;

        while out_x % 2 == 0 && out_y % 2 == 0 {
            out_x /= 2;
            out_y /= 2;
        }

        let mut cap = min(out_x.abs(), out_y.abs());

        let mut div = 3;
        while div <= cap {
            while out_x % div == 0 && out_y % div == 0 {
                out_x /= div;
                out_y /= div;

                cap = min(out_x.abs(), out_y.abs());
            }

            div += 2;
        }

        LineOfSight {
            x_diff: out_x,
            y_diff: out_y,
        }
    }
}

fn is_same_angle(a: LineOfSight, b: LineOfSight) -> bool {
    sign(a.x_diff) == sign(b.x_diff)
        && sign(a.y_diff) == sign(b.y_diff)
        && (a.x_diff * b.y_diff == a.y_diff * b.x_diff)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Sign {
    NEG,
    POS,
    ZERO,
}

impl Sign {
    fn to_int(self) -> i64 {
        match self {
            Sign::NEG => -1,
            Sign::POS => 1,
            Sign::ZERO => 0,
        }
    }
}

fn sign(a: i64) -> Sign {
    use Sign::*;

    if a < 0 {
        NEG
    } else if a > 0 {
        POS
    } else {
        ZERO
    }
}

impl PartialOrd for LineOfSight {
    fn partial_cmp(&self, other: &LineOfSight) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LineOfSight {
    fn cmp(&self, other: &LineOfSight) -> Ordering {
        let a = *self;
        let b = *other;

        let dir_a = to_dir(a);
        let dir_b = to_dir(b);

        if dir_a != dir_b {
            dir_a.cmp(&dir_b)
        } else {
            // now they're in the same quadrant
            if is_same_angle(a, b) {
                // least magnitude first
                a.mag().cmp(&b.mag())
            } else {
                // determines which is first, moving counterclockwise
                let cross = a.x_diff * b.y_diff - a.y_diff * b.x_diff;
                if cross > 0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
enum Direction {
    // Clockwise from north (this is important)
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

/// Params:
///     x_diff: me.x - other.x
///     y_diff: me.y - other.y
fn to_dir_vals(x_diff: i64, y_diff: i64) -> Direction {
    use Direction::*;
    use Sign::*;

    match sign(x_diff) {
        NEG => match sign(y_diff) {
            NEG => SE,
            ZERO => E,
            POS => NE,
        },
        ZERO => match sign(y_diff) {
            NEG => S,
            ZERO => unreachable!(),
            POS => N,
        },
        POS => match sign(y_diff) {
            NEG => SW,
            ZERO => W,
            POS => NW,
        },
    }
}

fn to_dir(loc: LineOfSight) -> Direction {
    to_dir_vals(loc.x_diff, loc.y_diff)
}

impl AsteroidField {
    pub fn all_vis(&self, my_x: usize, my_y: usize) -> Vec<LineOfSight> {
        // map from the canonical form to the particular LOS you want to use
        let mut can_see: BTreeMap<LineOfSight, LineOfSight> = BTreeMap::new();

        for (x, y) in self.locs.iter() {
            let (x, y) = (*x, *y);
            // not considered "detecting" it if you're on it
            if x == my_x && y == my_y {
                continue;
            }

            let los = LineOfSight {
                x_diff: (my_x as i64) - (x as i64),
                y_diff: (my_y as i64) - (y as i64),
            };

            let canonical = los.to_canonical();

            let entry = can_see.entry(canonical).or_insert(los);
            if los < *entry {
                *entry = los;
            }
        }

        let mut out = Vec::new();
        // important: BTreeMap iterates through KEYS in order
        for (_canonical, actual) in can_see {
            out.push(actual);
        }

        out
    }

    pub fn num_vis_from(&self, my_x: usize, my_y: usize) -> usize {
        self.all_vis(my_x, my_y).len()
    }
}

fn get_field(s: &str) -> AsteroidField {
    let mut locs = Vec::new();

    for (y, row) in s.trim().lines().enumerate() {
        for (x, c) in row.trim().chars().enumerate() {
            match c {
                '#' => {
                    locs.push((x, y));
                }
                '.' => {}
                _ => {
                    panic!("Unexpected character in field: {}", c);
                }
            }
        }
    }

    AsteroidField { locs }
}

#[derive(Debug, PartialEq, Eq)]
struct BestLocResult {
    #[allow(dead_code)] // used for unit test
    x: usize,
    #[allow(dead_code)] // used for unit test
    y: usize,
    num_seen: usize,
}

fn get_best_loc(map: &str) -> BestLocResult {
    let field = get_field(map);

    let mut best_loc = None;
    let mut best = None;

    for (x, y) in field.locs.iter() {
        let (x, y) = (*x, *y);
        let num_vis = field.num_vis_from(x, y);
        if best.is_none() || best.unwrap() < num_vis {
            best = Some(num_vis);
            best_loc = Some((x, y));
        }
    }

    let (x, y) = best_loc.unwrap();

    BestLocResult {
        x,
        y,
        num_seen: best.unwrap(),
    }
}

pub fn a() {
    let out = get_best_loc(DAY_10).num_seen;

    println!("10a: {}", out);
}

pub fn b() {
    let mut field = get_field(DAY_10);

    let out = get_best_loc(DAY_10);
    let (my_x, my_y) = (out.x, out.y);

    field.locs.retain(|(x, y)| *x != my_x || *y != my_y);

    let mut deleted: Vec<(usize, usize)> = Vec::new();

    while !field.locs.is_empty() {
        let can_see = field.all_vis(my_x, my_y);
        if can_see.is_empty() {
            panic!("Reached 'no visibility' before emptying field.");
        }
        
        let can_see: Vec<(usize, usize)> = can_see
            .into_iter()
            .map(|los| {
                let x = ((my_x as i64) - los.x_diff) as usize;
                let y = ((my_y as i64) - los.y_diff) as usize;
                (x, y)
            })
            .collect();

        for (x, y) in &can_see {
            deleted.push((*x, *y));
        }

        // TODO: This is bad use of data structures but I'm tired and it runs in 24ms and I'm going to bed
        field.locs.retain(|tup| !can_see.contains(tup));
    }

    let (x, y) = deleted[199];
    let ans = x * 100 + y;
    println!("10b: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_10_a() {
        let actual = get_best_loc(DAY_10);
        assert_eq!(actual.num_seen, 256);
    }

    #[test]
    fn test_sort_dirs() {
        use Direction::*;

        let mut actual = vec![NE, E, SE, SW, S, N, W, NW];

        actual.sort();

        let expected = vec![N, NE, E, SE, S, SW, W, NW];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sort() {
        use Direction::*;

        let field = AsteroidField {
            locs: vec![
                (1, 1),
                (5, 2),
                (4, 1),
                (5, 1),
                (7, 7), // obscured by (5, 5), later
                (5, 3),
                (3, 1),
                (5, 5),
                (2, 5),
                (1, 4),
                (1, 2),
            ],
        };

        let (my_x, my_y) = (3, 3);

        let actual_transposes: Vec<(i64, i64)> = field
            .locs
            .iter()
            .map(|(x, y)| (my_x as i64 - (*x as i64), my_y as i64 - (*y as i64)))
            .collect();

        let expected_transposes = vec![
            (2, 2),   // (1, 1),
            (-2, 1),  // (5, 2),
            (-1, 2),  // (4, 1),
            (-2, 2),  // (5, 1),
            (-4, -4), // (7, 7),
            (-2, 0),  // (5, 3),
            (0, 2),   // (3, 1),
            (-2, -2), // (5, 5),
            (1, -2),  // (2, 5),
            (2, -1),  // (1, 4),
            (2, 1),   // (1, 2),
        ];

        assert_eq!(
            actual_transposes, expected_transposes,
            "Testing that transposition works"
        );

        let actual_dirs: Vec<Direction> = expected_transposes
            .iter()
            .map(|(x, y)| to_dir_vals(*x, *y))
            .collect();
        let expected_dirs = vec![NW, NE, NE, NE, SE, E, N, SE, SW, SW, NW];

        assert_eq!(actual_dirs, expected_dirs, "Testing to directions work");

        let actual: Vec<(i64, i64)> = field
            .all_vis(my_x, my_y)
            .into_iter()
            .map(|los| (los.x_diff, los.y_diff))
            .collect();

        // order very important
        let expected = vec![
            // N
            (0, 2), // (3, 1),
            // NE
            (-1, 2), // (4, 1),
            (-2, 2), // (5, 1),
            (-2, 1), // (5, 2),
            // E
            (-2, 0), // (5, 3),
            // SE
            (-2, -2), // (5, 5),
            // obscured: (-4, -4), // (7, 7), hidden by (-2, -2)
            // S
            // SW
            (1, -2), // (2, 5),
            (2, -1), // (1, 4),
            // W
            // NW
            (2, 1), // (1, 2),
            (2, 2), // (1, 1),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_a_1() {
        let field = "
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####";

        let actual = get_best_loc(&field);
        let expected = BestLocResult {
            x: 5,
            y: 8,
            num_seen: 33,
        };

        assert_eq!(actual, expected)
    }
}
