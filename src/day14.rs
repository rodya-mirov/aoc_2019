use std::collections::HashMap;
use std::iter::Peekable;

const DAY_14: &str = include_str!("resources/14a.txt");

/// Made into a module so I can collapse it, it's not that interesting
mod parse {
    use super::*;

    fn read_num<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> i64 {
        let mut out = 0;
        loop {
            match iter.peek().copied() {
                None => return out,
                Some(c) => {
                    if c.is_digit(10) {
                        out = out * 10 + c.to_digit(10).unwrap() as i64;
                        iter.next();
                    } else {
                        return out;
                    }
                }
            }
        }
    }

    fn read_id<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> String {
        let mut out = String::new();

        loop {
            match iter.peek().copied() {
                None => return out,
                Some(c) => {
                    if c.is_ascii_alphabetic() {
                        out.push(c);
                        iter.next();
                    } else {
                        return out;
                    }
                }
            }
        }
    }

    fn read_char<T: Iterator<Item = char>>(iter: &mut T, expected: char) {
        let next = iter.next();
        assert_eq!(next, Some(expected));
    }

    pub(super) fn str_to_reactions(data: &str) -> Reactions {
        let mut out = Reactions::new();

        // I tried to do this in regex and I cried and now I'm sad
        for line in data.trim().lines() {
            let mut line_chars_iter = line.trim().chars().peekable();

            let mut ingredients = Vec::new();

            let ingredient_num = read_num(&mut line_chars_iter);
            read_char(&mut line_chars_iter, ' ');
            let ingredient_name = read_id(&mut line_chars_iter);

            ingredients.push((ingredient_name, ingredient_num));

            while line_chars_iter.peek().copied() == Some(',') {
                read_char(&mut line_chars_iter, ',');
                read_char(&mut line_chars_iter, ' ');

                let ingredient_num = read_num(&mut line_chars_iter);
                read_char(&mut line_chars_iter, ' ');
                let ingredient_name = read_id(&mut line_chars_iter);

                ingredients.push((ingredient_name, ingredient_num));
            }

            read_char(&mut line_chars_iter, ' ');
            read_char(&mut line_chars_iter, '=');
            read_char(&mut line_chars_iter, '>');
            read_char(&mut line_chars_iter, ' ');

            let goal_num = read_num(&mut line_chars_iter);
            read_char(&mut line_chars_iter, ' ');
            let goal_name = read_id(&mut line_chars_iter);
            assert_eq!(line_chars_iter.next(), None);

            out.register_reaction(ingredients, (goal_name, goal_num));
        }

        out
    }
}

use parse::str_to_reactions;

const ROOT_ELEMENT: &str = "ORE";

#[derive(Clone, Eq, PartialEq, Debug)]
struct Reactions {
    elt_lookups: HashMap<String, usize>,
    // goal element id -> (# produced, list of things required to produce it)
    // assumes there is a unique reaction for each element
    reactions: HashMap<usize, (i64, Vec<EltMult>)>,
}

impl Reactions {
    fn new() -> Self {
        Reactions {
            elt_lookups: HashMap::new(),
            reactions: HashMap::new(),
        }
    }

    fn register_reaction(&mut self, ingredients: Vec<(String, i64)>, goal: (String, i64)) {
        let (goal_name, goal_mult) = goal;
        let goal_id = self.register_element(goal_name);

        let mut reaction: Vec<EltMult> = Vec::new();

        for (elt_name, mult) in ingredients {
            let elt_id = self.register_element(elt_name);
            let em = EltMult { elt_id, mult };

            reaction.push(em);
        }

        let old_reaction = self.reactions.insert(goal_id, (goal_mult, reaction));
        assert!(
            old_reaction.is_none(),
            "Should not have a storied reaction for element"
        );
    }

    fn register_element(&mut self, element_name: String) -> usize {
        let new_id = self.elt_lookups.len();
        *self.elt_lookups.entry(element_name).or_insert(new_id)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct EltMult {
    elt_id: usize,
    mult: i64,
}

fn ore_for_fuel(reactions: &Reactions, goal_elt: &str, goal_amt: i64) -> i64 {
    assert!(goal_amt > 0);
    // println!("Reaction setup: {:?}", reactions);

    let mut resources: HashMap<usize, i64> = HashMap::new();

    let mut ore_required: i64 = 0;
    let ore_id: usize = reactions.elt_lookups.get(ROOT_ELEMENT).copied().unwrap();

    resources.insert(*reactions.elt_lookups.get(goal_elt).unwrap(), -goal_amt);

    // Note: this is super sloppy without a topological sort but at this data scale I just
    // don't care ... I feel kind of guilty, though.

    loop {
        // println!("Current state: {:?}", resources);

        // First make sure we didn't get any ore in there ...
        if resources.contains_key(&ore_id) {
            ore_required -= resources.remove(&ore_id).unwrap();
        }

        // First, just grab SOMETHING we don't have enough of
        let next_need = resources
            .iter()
            .filter(|(_key, val)| **val < 0)
            .map(|(key, val)| (*key, *val))
            .next();

        if next_need.is_none() {
            break;
        }

        let (elt_id, elt_quant) = next_need.unwrap();
        let need = -elt_quant;
        assert!(need > 0);

        let (amt_get, reaction) = reactions.reactions.get(&elt_id).unwrap();
        let mut num_reactions = need / amt_get;
        if num_reactions * amt_get < need {
            num_reactions += 1;
        }
        *resources.entry(elt_id).or_insert(0) += amt_get * num_reactions;

        for em in reaction {
            *resources.entry(em.elt_id).or_insert(0) -= em.mult * num_reactions;
        }
    }

    ore_required
}

/// Get the maximum amount of fuel you can produce for a certain amount of ore
fn fuel_for_ore(data: &str, goal_elt: &str, ore_cap: i64) -> i64 {
    let reactions = str_to_reactions(data);

    assert!(ore_cap >= 0);

    // Condition maintained: fuel_min is ALWAYS possible to achieve
    let mut fuel_min = 0;
    let mut fuel_max = 1;

    while ore_for_fuel(&reactions, goal_elt, fuel_max) <= ore_cap {
        fuel_min = fuel_max;
        fuel_max *= 2;
    }

    // Condition maintained from now on; fuel_max is ALWAYS impossible to achieve

    while fuel_min + 1 < fuel_max {
        let fuel_mid = (fuel_min + fuel_max) / 2;
        let ore_req = ore_for_fuel(&reactions, goal_elt, fuel_mid);

        if ore_req <= ore_cap {
            fuel_min = fuel_mid;
        } else {
            fuel_max = fuel_mid;
        }
    }

    fuel_min
}

pub fn a() {
    let reactions = str_to_reactions(DAY_14);
    let out = ore_for_fuel(&reactions, "FUEL", 1);

    println!("14a: {}", out);
}

pub fn b() {
    let fuel_cap = fuel_for_ore(DAY_14, "FUEL", 1000000000000);

    println!("14b: {}", fuel_cap);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest_test() {
        let data = "
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";

        let reactions = str_to_reactions(data);

        let required = ore_for_fuel(&reactions, "FUEL", 1);

        assert_eq!(required, 165);
    }
}
