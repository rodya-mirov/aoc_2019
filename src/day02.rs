const DATA_2A: &str = include_str!("resources/2a.txt");

fn get_ints() -> Vec<i64> {
    DATA_2A
        .split(',')
        .filter(|token| token.len() > 0)
        .map(|token| token.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn run_program(data: &mut Vec<i64>) {
    let mut ip = 0;
    loop {
        let op_code = data[ip];

        match op_code {
            1 => {
                let a = data[data[ip + 1] as usize];
                let b = data[data[ip + 2] as usize];

                let result_register = data[ip + 3] as usize;

                data[result_register] = a + b;
                ip += 4;
            }
            2 => {
                let a = data[data[ip + 1] as usize];
                let b = data[data[ip + 2] as usize];

                let result_register = data[ip + 3] as usize;

                data[result_register] = a * b;
                ip += 4;
            }
            99 => return,
            _ => {
                panic!("Unrecognized opcode {}", op_code);
            }
        }
    }
}

pub fn a() {
    let mut data = get_ints();
    data[1] = 12;
    data[2] = 2;

    run_program(&mut data);

    println!("2a: {}", data[0]);
}

pub fn b() {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut data = get_ints();
            data[1] = noun;
            data[2] = verb;

            run_program(&mut data);

            if data[0] == 19690720 {
                let answer = 100 * noun + verb;
                println!("2b: {}", answer);
                return;
            }
        }
    }

    panic!("No noun/verb gave the correct answer");
}
