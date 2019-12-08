const DAY_7: &str = include_str!("resources/7a.txt");

use crate::intcode::{str_to_ints, VecInput, VecOutput, VM};

fn run_amplifiers(code: &[i64], phases: [i64; 5]) -> i64 {
    let mut out_so_far = 0;
    for &phase in &phases {
        let code: Vec<i64> = code.iter().copied().collect();
        // println!("Amp code: {:?}", code);
        let mut amp = VM::new(code);
        let mut inp = VecInput::new(vec![phase, out_so_far]);
        let mut out = VecOutput::new();
        // println!("Input to amplifier is {:?}", inp);

        assert_eq!(inp.get_counter(), 0);

        amp.run(&mut inp, &mut out);

        // println!("Output of amplifier is {:?}", out);

        assert_eq!(inp.get_counter(), 2);

        let out_vec = out.to_vec();
        assert_eq!(out_vec.len(), 1);
        // println!();

        out_so_far = out_vec[0];
    }

    out_so_far
}

fn do_7a() -> i64 {
    let code = str_to_ints(DAY_7);
    let mut best_out = None;

    for p1 in 0..5 {
        for p2 in 0..5 {
            if [p1].contains(&p2) {
                continue;
            }

            for p3 in 0..5 {
                if [p1, p2].contains(&p3) {
                    continue;
                }

                for p4 in 0..5 {
                    if [p1, p2, p3].contains(&p4) {
                        continue;
                    }

                    for p5 in 0..5 {
                        if [p1, p2, p3, p4].contains(&p5) {
                            continue;
                        }

                        let phases = [p1, p2, p3, p4, p5];
                        let output = run_amplifiers(&code, phases);
                        best_out = best_out.map(|n| std::cmp::max(n, output)).or(Some(output));
                    }
                }
            }
        }
    }

    best_out.unwrap()
}

pub fn a() {
    println!("7a: {}", do_7a());
}

pub fn b() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7a() {
        assert_eq!(do_7a(), 51679);
    }

    #[test]
    fn test_a_1() {
        let phases = [4, 3, 2, 1, 0];
        let code = str_to_ints("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");

        let out = run_amplifiers(&code, phases);

        assert_eq!(out, 43210);
    }

    #[test]
    fn test_a_2() {
        let phases = [0, 1, 2, 3, 4];
        let code =
            str_to_ints("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");

        let out = run_amplifiers(&code, phases);

        assert_eq!(out, 54321);
    }

    #[test]
    fn test_a_3() {
        let phases = [1, 0, 4, 3, 2];
        let code = str_to_ints("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");

        let out = run_amplifiers(&code, phases);

        assert_eq!(out, 65210);
    }
}
