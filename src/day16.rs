const DAY_16: &str = include_str!("resources/16a.txt");

fn str_to_ints(data: &str) -> Vec<i64> {
    data.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

fn fft(data: &[i64]) -> Vec<i64> {
    let mut out = Vec::with_capacity(data.len());

    const PHASES: [i64; 4] = [0, 1, 0, -1];

    for phase in 1..=data.len() {
        let mut phase_index = 0;
        let mut change_index = 0;
        let mut total = 0;

        for num in data {
            change_index += 1;
            while change_index >= phase {
                change_index = 0;
                phase_index += 1;
                while phase_index >= 4 {
                    phase_index -= 4;
                }
            }

            let phase_mult = PHASES[phase_index];
            total += num * phase_mult;
        }

        let next = (if total > 0 { total } else { -total }) % 10;
        out.push(next);
    }

    out
}

fn lots_of_fft(data: &[i64]) -> i64 {
    let mut res = fft(data);
    for _ in 1..100 {
        res = fft(&res);
    }

    let slice = &res[0..8];
    let mut total = 0;
    for n in slice {
        total = 10 * total + *n;
    }

    total
}

pub fn a() {
    let input_data = str_to_ints(DAY_16);

    let total = lots_of_fft(&input_data);

    println!("16a: {:?}", total);
}

pub fn b() {
    // I heard you like project euler
    // Do some math you lazy twit

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mods_suck() {
        let a = -3;
        let b = 10;
        let actual = ((a % b) + b) % b;
        assert_eq!(actual, 7);
    }

    #[test]
    fn simple_test_a() {
        let start = [1, 2, 3, 4, 5, 6, 7, 8];

        let one = fft(&start);
        let two = fft(&one);
        let three = fft(&two);

        assert_eq!(one, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        assert_eq!(two, vec![3, 4, 0, 4, 0, 4, 3, 8]);
        assert_eq!(three, vec![0, 3, 4, 1, 5, 5, 1, 8]);
    }
}
