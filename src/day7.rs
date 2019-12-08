use std::sync::mpsc::{channel, Receiver, Sender};

const DAY_7: &str = include_str!("resources/7a.txt");

use crate::intcode::{str_to_ints, VMInput, VMOutput, VecInput, VecOutput, VM};

pub struct MessageInput {
    pub rx: Receiver<i64>,
}

impl VMInput for MessageInput {
    fn get_input(&mut self) -> i64 {
        // Blocks; so this will need to be on different threads
        // Note that if there is no response for 50 ms the thread explodes
        // dramatic but fine for a puzzle
        self.rx.recv_timeout(std::time::Duration::from_millis(100)).unwrap()
    }
}

pub struct MessageOutput {
    pub name: &'static str,
    pub tx: Sender<i64>,
}

impl VMOutput for MessageOutput {
    fn do_output(&mut self, val: i64) {
        // println!("Sending {} from VM {}", val, self.name);
        self.tx.send(val).unwrap()
    }
}

pub struct FinalOutput {
    pub name: &'static str,
    pub feedback_tx: Sender<i64>,
    pub scribe_tx: Sender<i64>,
}

impl VMOutput for FinalOutput {
    fn do_output(&mut self, val: i64) {
        // println!("Sending {} from VM {}", val, self.name);
        self.scribe_tx.send(val).unwrap();
        if let Err(_e) = self.feedback_tx.send(val) {
            // println!("Error sending feedback tx; probably origin VM has terminated: {}", _e);
        }
    }
}

fn run_amps_serial(code: &[i64], phases: [i64; 5]) -> i64 {
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

    let (tx_a, rx_a) = channel();
    tx_a.send(phases[0]).unwrap();
    tx_a.send(0).unwrap();

    let (tx_b, rx_b) = channel();
    tx_b.send(phases[1]).unwrap();

    let (tx_c, rx_c) = channel();
    tx_c.send(phases[2]).unwrap();

    let (tx_d, rx_d) = channel();
    tx_d.send(phases[3]).unwrap();

    let (tx_e, rx_e) = channel();
    tx_e.send(phases[4]).unwrap();

    // additional capture for the output of the final amp (E)
    let (scribe_tx, scribe_rx) = channel();

    let mut output_a = MessageOutput {
        name: "A",
        tx: tx_b,
    };
    let mut output_b = MessageOutput {
        name: "B",
        tx: tx_c,
    };
    let mut output_c = MessageOutput {
        name: "C",
        tx: tx_d,
    };
    let mut output_d = MessageOutput {
        name: "D",
        tx: tx_e,
    };
    let mut output_e = FinalOutput {
        name: "E",
        scribe_tx,
        feedback_tx: tx_a,
    };

    let mut input_a = MessageInput { rx: rx_a };
    let mut input_b = MessageInput { rx: rx_b };
    let mut input_c = MessageInput { rx: rx_c };
    let mut input_d = MessageInput { rx: rx_d };
    let mut input_e = MessageInput { rx: rx_e };

    let code_a = code.iter().copied().collect();
    let _handle_a = std::thread::spawn(move || {
        // println!("VM A started");
        let mut vm = VM::new(code_a);
        vm.run(&mut input_a, &mut output_a);
        // println!("VM A terminated");
    });

    let code_b = code.iter().copied().collect();
    let _handle_b = std::thread::spawn(move || {
        // println!("VM B started");
        let mut vm = VM::new(code_b);
        vm.run(&mut input_b, &mut output_b);
        // println!("VM B terminated");
    });

    let code_c = code.iter().copied().collect();
    let _handle_c = std::thread::spawn(move || {
        // println!("VM C started");
        let mut vm = VM::new(code_c);
        vm.run(&mut input_c, &mut output_c);
        // println!("VM C terminated");
    });

    let code_d = code.iter().copied().collect();
    let _handle_d = std::thread::spawn(move || {
        // println!("VM D started");
        let mut vm = VM::new(code_d);
        vm.run(&mut input_d, &mut output_d);
        // println!("VM D terminated");
    });

    let code_e = code.iter().copied().collect();
    let handle_e = std::thread::spawn(move || {
        // println!("VM E started");
        let mut vm = VM::new(code_e);
        vm.run(&mut input_e, &mut output_e);
        // println!("VM E terminated");
    });

    handle_e.join().unwrap();

    let mut last_msg = None;
    while let Ok(msg) = scribe_rx.recv() {
        last_msg = Some(msg);
    }

    // NB: this all depends on the code above being nice and well-constructed; otherwise
    // the child threads will spinlock forever and do badness
    last_msg.unwrap()
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
