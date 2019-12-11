const DATA_1A: &str = include_str!("resources/1a.txt");

fn get_ints() -> Vec<i64> {
    DATA_1A
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn fuel_cost(weight: i64) -> i64 {
    if weight < 6 {
        0
    } else {
        (weight / 3) - 2
    }
}

fn total_fuel_cost(weight: i64) -> i64 {
    let mut total = 0;
    let mut to_fuel = weight;

    loop {
        let fuel_fuel = fuel_cost(to_fuel);
        if fuel_fuel == 0 {
            return total;
        } else {
            total += fuel_fuel;
            to_fuel = fuel_fuel;
        }
    }
}

pub fn a() {
    let total: i64 = get_ints().into_iter().map(fuel_cost).sum();

    println!("1A: {}", total);
}

pub fn b() {
    let total: i64 = get_ints().into_iter().map(total_fuel_cost).sum();

    println!("1B: {}", total);
}
