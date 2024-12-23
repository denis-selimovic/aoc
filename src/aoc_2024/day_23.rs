use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::plugin::Plugin;
use crate::reader::Reader;


pub struct AoC2024Day23;


struct Graph {
    adj_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: &String, v: &String) {
        self.adj_list.entry(u.clone()).or_insert_with(HashSet::new).insert(v.clone());
        self.adj_list.entry(v.clone()).or_insert_with(HashSet::new).insert(u.clone());
    }

    fn nodes(&self) -> Vec<String> {
        self.adj_list
            .keys()
            .cloned()
            .collect::<Vec<_>>()
    }

    fn are_neighbours(&self, u: &String, v: &String) -> bool {
        self.adj_list
            .get(u)
            .cloned()
            .unwrap_or_else(HashSet::new)
            .contains(v)
    }

    fn neighbours(&self, u: &String) -> HashSet<String> {
        self.adj_list
            .get(u)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
}

fn parse_input(content: &String) -> Graph {
    let mut graph = Graph::new();

    for line in content.lines() {
        let nodes = line
            .split("-")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        graph.add_edge(&nodes[0], &nodes[1]);
    }

    graph
}

fn bron_kerbosch(
    graph: &Graph,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    max_clique: &mut HashSet<String>,
) {
    if p.is_empty() && x.is_empty() {
        // Found a maximal clique
        if r.len() > max_clique.len() {
            *max_clique = r;
        }
        return;
    }

    // Pivoting: Choose a pivot to minimize recursive calls
    let pivot = p.union(&x).next().cloned().unwrap();

    // Nodes to consider (P \ N(pivot))
    let candidates: HashSet<_> = p.difference(&graph.neighbours(&pivot)).cloned().collect();

    for v in candidates {
        let mut r_new = r.clone();
        r_new.insert(v.clone());

        let neighbors = graph.neighbours(&v);
        let p_new: HashSet<_> = p.intersection(&neighbors).cloned().collect();
        let x_new: HashSet<_> = x.intersection(&neighbors).cloned().collect();

        bron_kerbosch(graph, r_new, p_new, x_new, max_clique);

        p.remove(&v);
        x.insert(v);
    }
}

fn part1(graph: &Graph) -> u64 {
    let mut nodes = graph.nodes();
    nodes.sort();
    let mut three_cliques: HashSet<(String, String, String)> = HashSet::new();

    for (u, v) in nodes.iter().combinations(2).map(|c| (c[0], c[1])) {
        if !graph.are_neighbours(u, v) {
            continue;
        }

        let uu = graph.neighbours(u);
        let vv = graph.neighbours(v);

        for t in uu.intersection(&vv) {
            if t < u {
                three_cliques.insert((t.clone(), u.clone(), v.clone()));
            }
            else if t > v {
                three_cliques.insert((u.clone(), v.clone(), t.clone()));
            } else {
                three_cliques.insert((u.clone(), t.clone(), v.clone()));
            }
        }
    }


    three_cliques
        .into_iter()
        .filter(|c| c.0.starts_with('t') || c.1.starts_with('t') || c.2.starts_with('t'))
        .collect::<HashSet<_>>()
        .len() as u64
}

fn part2(graph: &Graph) -> u64 {
    let r = HashSet::new();
    let p = graph.nodes().iter().map(|c| c.to_string()).collect::<HashSet<_>>();
    let x = HashSet::new();
    let mut max_clique = HashSet::new();

    bron_kerbosch(graph, r, p, x, &mut max_clique);
    let mut nodes = max_clique.iter().collect::<Vec<_>>();
    nodes.sort();
    
    println!("{}", nodes.iter().map(|s| s.to_string()).join(","));

    max_clique.len() as u64
}

impl Plugin for AoC2024Day23 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(23, 2024);
        let puzzle = reader.load_puzzle();
        let graph = parse_input(&puzzle);

        (part1(&graph), part2(&graph))
    }
}
