const DAY_7: &str = include_str!("resources/7a.txt");

use crate::intcode::{str_to_ints, RunResult, VM};

fn run_amps_serial(code: &[i64], phases: [i64; 5]) -> i64 {
    let mut out_so_far = 0;
    for &phase in &phases {
        // println!("Amp code: {:?}", code);
        let mut amp = VM::new(code);
        amp.give_input(phase);
        amp.give_input(out_so_far);
        // println!("Input to amplifier is {:?}", inp);

        let result = amp.run();
        assert_eq!(result, RunResult::Stopped);

        // println!("Output of amplifier is {:?}", out);

        let out = amp.get_all_outputs();
        assert_eq!(out.len(), 1);
        // println!();

        out_so_far = out[0];
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
                        let output = run_amps_serial(&code, phases);
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

fn do_amps_feedback(code: &[i64], phases: [i64; 5]) -> i64 {
    // println!("Trying phases {:?}", phases);

    let mut amps = Vec::with_capacity(5);
    for &phase in &phases {
        let mut amp = VM::new(code);
        amp.give_input(phase);
        amps.push(amp);
    }

    amps[0].give_input(0);

    let mut last_outputs = Vec::new();

    while !amps[4].is_stopped() {
        let mut change_happened = false;
        for i in 0..4 {
            amps[i].run();
            while let Some(output) = amps[i].get_next_output() {
                change_happened = true;
                amps[i + 1].give_input(output);
            }
        }

        amps[4].run();
        while let Some(output) = amps[4].get_next_output() {
            change_happened = true;
            last_outputs.push(output);
            amps[0].give_input(output);
        }

        if !change_happened {
            panic!("All amplifiers are waiting for input, so the feedback loop cannot progress");
        }
    }

    last_outputs.pop().unwrap()
}

fn do_7b() -> i64 {
    let code = str_to_ints(DAY_7);
    let mut best_out = None;

    for p1 in 5..10 {
        for p2 in 5..10 {
            if [p1].contains(&p2) {
                continue;
            }

            for p3 in 5..10 {
                if [p1, p2].contains(&p3) {
                    continue;
                }

                for p4 in 5..10 {
                    if [p1, p2, p3].contains(&p4) {
                        continue;
                    }

                    for p5 in 5..10 {
                        if [p1, p2, p3, p4].contains(&p5) {
                            continue;
                        }

                        let phases = [p1, p2, p3, p4, p5];
                        let output = do_amps_feedback(&code, phases);
                        best_out = best_out.map(|n| std::cmp::max(n, output)).or(Some(output));
                    }
                }
            }
        }
    }

    best_out.unwrap()
}

pub fn b() {
    println!("7b: {}", do_7b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7a() {
        assert_eq!(do_7a(), 51679);
    }

    #[test]
    fn test_7b() {
        assert_eq!(do_7b(), 19539216);
    }

    #[test]
    fn test_a_1() {
        let phases = [4, 3, 2, 1, 0];
        let code = str_to_ints("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");

        let out = run_amps_serial(&code, phases);

        assert_eq!(out, 43210);
    }

    #[test]
    fn test_a_2() {
        let phases = [0, 1, 2, 3, 4];
        let code =
            str_to_ints("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");

        let out = run_amps_serial(&code, phases);

        assert_eq!(out, 54321);
    }

    #[test]
    fn test_a_3() {
        let phases = [1, 0, 4, 3, 2];
        let code = str_to_ints("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");

        let out = run_amps_serial(&code, phases);

        assert_eq!(out, 65210);
    }
}
