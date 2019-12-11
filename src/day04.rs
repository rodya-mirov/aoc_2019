use std::collections::HashSet;

const DATA_4A: &str = include_str!("resources/4a.txt");
const NUM_DIGITS: usize = 6;

fn get_range() -> [usize; 2] {
    let text = DATA_4A.split('-').collect::<Vec<_>>();
    assert_eq!(text.len(), 2);

    [
        text.get(0).unwrap().parse().unwrap(),
        text.get(1).unwrap().parse().unwrap(),
    ]
}

fn get_all_nondecreasing(
    set: &mut HashSet<usize>,
    has_repeat: bool,
    running: usize,
    // between 0 and 9 (inclusive)
    last_digit: usize,
    // positive, always
    digits_rem: usize,
) {
    if digits_rem == 0 {
        // nothing
    } else if digits_rem == 1 {
        if has_repeat {
            for d in last_digit..10 {
                let next = running * 10 + d;
                set.insert(next);
            }
        } else {
            set.insert(running * 10 + last_digit);
        }
    } else {
        if last_digit > 0 {
            get_all_nondecreasing(
                set,
                true,
                running * 10 + last_digit,
                last_digit,
                digits_rem - 1,
            );
        }
        for d in last_digit + 1..10 {
            get_all_nondecreasing(set, has_repeat, running * 10 + d, d, digits_rem - 1);
        }
    }
}

pub fn a() {
    let [start, end] = get_range();

    let mut all = HashSet::new();

    get_all_nondecreasing(&mut all, false, 0, 0, NUM_DIGITS);

    let count = all.into_iter().filter(|&n| n >= start && n <= end).count();

    println!("4a: {}", count);
}

fn has_good_pair(mut n: usize) -> bool {
    let mut digit_counts: [usize; 10] = [0; 10];

    while n > 0 {
        digit_counts[n % 10] += 1;
        n /= 10;
    }

    digit_counts.iter().any(|&n| n == 2)
}

pub fn b() {
    let [start, end] = get_range();

    let mut all = HashSet::new();

    get_all_nondecreasing(&mut all, false, 0, 0, NUM_DIGITS);

    let count = all
        .into_iter()
        .filter(|&n| n >= start && n <= end && has_good_pair(n))
        .count();

    println!("4b: {}", count);
}
