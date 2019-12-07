use std::collections::HashMap;

const DAY_6: &str = include_str!("resources/6a.txt");

struct GraphBuilder {
    // node name to node id
    name_to_id: HashMap<String, usize>,
    // node id to parent node id
    parents: HashMap<usize, usize>,
    // node id to orbital depth
    orbital_depth: HashMap<usize, usize>,
}

struct Graph {
    // node name to node id
    name_to_id: HashMap<String, usize>,
    // node id to node name, for debugging
    id_to_name: HashMap<usize, String>,
    // node id to parent node id
    parents: HashMap<usize, usize>,
    // node id to orbital depth
    orbital_depth: HashMap<usize, usize>,
}

impl GraphBuilder {
    fn new() -> Self {
        GraphBuilder {
            name_to_id: HashMap::new(),
            parents: HashMap::new(),
            orbital_depth: HashMap::new(),
        }
    }

    fn build(mut self) -> Graph {
        let node_ids = self
            .name_to_id
            .iter()
            .map(|(_, &id)| id)
            .collect::<Vec<usize>>();

        // Initialize the orbital depth map
        for node_id in node_ids {
            self.get_orbital_depth(node_id);
        }

        let mut id_to_name: HashMap<usize, String> = HashMap::new();
        for (node_name, &node_id) in &self.name_to_id {
            id_to_name.insert(node_id, node_name.to_string());
        }

        Graph {
            name_to_id: self.name_to_id,
            id_to_name,
            parents: self.parents,
            orbital_depth: self.orbital_depth,
        }
    }

    fn get_id(&mut self, node_name: String) -> usize {
        let next_id = self.name_to_id.len();
        *self.name_to_id.entry(node_name).or_insert(next_id)
    }

    fn add_parent_reln(&mut self, child_name: String, parent_name: String) {
        let child_id = self.get_id(child_name);
        let parent_id = self.get_id(parent_name);
        self.parents.insert(child_id, parent_id);
    }

    fn get_orbital_depth(&mut self, node_id: usize) -> usize {
        if let Some(&depth) = self.orbital_depth.get(&node_id) {
            return depth;
        }

        let depth = {
            if self.parents.contains_key(&node_id) {
                let parent_id = *self.parents.get(&node_id).unwrap();
                1 + self.get_orbital_depth(parent_id)
            } else {
                0
            }
        };

        self.orbital_depth.insert(node_id, depth);
        depth
    }
}

impl Graph {
    fn get_num_trans_orbits(&self) -> usize {
        self.orbital_depth.iter().map(|(_, depth)| *depth).sum()
    }

    fn orbital_distance(&self, a: &str, b: &str) -> usize {
        let mut a_id = *self.name_to_id.get(a).unwrap();
        let mut b_id = *self.name_to_id.get(b).unwrap();

        let mut a_depth = *self.orbital_depth.get(&a_id).unwrap();
        let mut b_depth = *self.orbital_depth.get(&b_id).unwrap();

        let mut dist = 0;

        while a_depth > b_depth {
            a_id = *self.parents.get(&a_id).unwrap();
            a_depth = *self.orbital_depth.get(&a_id).unwrap();
            dist += 1;
        }

        while a_depth < b_depth {
            b_id = *self.parents.get(&b_id).unwrap();
            b_depth = *self.orbital_depth.get(&b_id).unwrap();
            dist += 1;
        }
        assert_eq!(a_depth, b_depth);

        while a_id != b_id {
            a_id = *self.parents.get(&a_id).unwrap_or_else(|| {
                panic!("Node {}, whose name is {}, has no parent!", a_id, self.id_to_name.get(&a_id).unwrap())
            });
            dist += 1;

            b_id = *self.parents.get(&b_id).unwrap();
            dist += 1;
        }

        dist
    }
}

fn get_graph(text: &str) -> Graph {
    let mut graph = GraphBuilder::new();

    for row in text.lines() {
        let tokens = row.split(')').collect::<Vec<&str>>();
        assert_eq!(tokens.len(), 2);
        let a = tokens[0].trim().to_string();
        let b = tokens[1].trim().to_string();

        graph.add_parent_reln(b.to_string(), a.to_string());
    }

    graph.build()
}

pub fn a() {
    let graph = get_graph(DAY_6);
    let num_orbits = graph.get_num_trans_orbits();
    println!("6a: {}", num_orbits);
}

pub fn b() {
    let graph = get_graph(DAY_6);

    let you = "YOU";
    let san = "SAN";

    // the -2 is for "moving to orbiting to the same thing" instead of moving you -> san
    let dist = graph.orbital_distance(you, san) - 2;

    println!("6b: {}", dist);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        let sample = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";

        let graph = get_graph(sample);

        assert_eq!(0, graph.orbital_distance("YOU", "YOU"));

        assert_eq!(6, graph.orbital_distance("YOU", "SAN"));
    }
}
