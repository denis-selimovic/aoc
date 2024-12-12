use std::collections::{HashMap, HashSet};

use num_complex::Complex;

use crate::plugin::Plugin;
use crate::reader::Reader;

pub struct AoC2024Day12;


fn edge(ps: &Vec<Complex<i32>>) -> (u64, u64) {
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();

    for p in ps {
        for d in vec![Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, -1), Complex::new(0, 1)] {
            if !ps.contains(&(p + d)) {
                s1.insert((*p, d));
            }
        }
    }

    for (p, d) in &s1 {
        s2.insert((p + d * Complex::new(0, 1), *d));
    }

    let diff = s1.difference(&s2).map(|c| *c).collect::<HashSet<(Complex<i32>, Complex<i32>)>>();

    (s1.len() as u64, diff.len() as u64)
}


impl Plugin for AoC2024Day12 {
    fn execute(&self) -> (u64, u64) {
        let reader = Reader::new(12, 2024);
        let puzzle = reader.load_puzzle();

        let grid = reader.to_grid(&puzzle);
        let mut cgrid: HashMap<Complex<i32>, String> = HashMap::new();
        let mut union_find: HashMap<Complex<i32>, HashSet<Complex<i32>>> = HashMap::new();
        
        for (i, line) in grid.into_iter().enumerate() {
            for (j, ch) in line.into_iter().enumerate() {
                cgrid.insert(Complex::new(i as i32, j as i32), ch);
            }
        }

        for key in cgrid.keys() {
            let mut set = HashSet::new();
            set.insert(key.clone());
            union_find.insert(key.clone(), set);
        }

        for (p, pch) in &cgrid {
            for n in vec![*p + Complex::new(1, 0), p + Complex::new(-1 ,0), p + Complex::new(0, 1), p + Complex::new(0, -1)] {
                if let Some(nch) = cgrid.get(&n) {
                    if nch == pch {
                        if let Some(sets_p) = union_find.get(p) {
                            if let Some(sets_n) = union_find.get(&n) {
                                let union = sets_p.union(sets_n).map(|c| *c).collect::<HashSet<Complex<i32>>>();
                                union_find.insert(*p, union.clone());

                                for x in &union {
                                    union_find.insert(*x, union.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        let sets: HashSet<_> = union_find.values().cloned().map(|s| s.into_iter().collect::<Vec<_>>()).collect();
        let (mut part1, mut part2) = (0, 0);

        for s in &sets {
            let (perimeter, edges) = edge(s);
            part1 += (s.len() as u64) * perimeter;
            part2 += (s.len() as u64) * edges;
        }

        (part1, part2)
    }
}
