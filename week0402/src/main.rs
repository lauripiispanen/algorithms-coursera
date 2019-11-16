#![feature(test)]
mod read_file;

extern crate test;

extern crate clap;
use clap::{Arg, App};
use std::collections::HashMap;
use core::hash::{Hasher, BuildHasherDefault};

use itertools::Itertools;

fn main() {
    let arg_matches = App::new("TSP with dynamic programming (Held-Karp algorithm)")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();

    let mut i = read_file::read_numeric_lines::<f64>(input_file);

    let _:usize = i.next().map(|v| v[0] as usize).unwrap();
    //let n_vertices = num_vertices.try_into().unwrap();

    let vertices:Vec<Vertex> = i.map(|v| (v[0], v[1])).collect();

    held_karp(create_distance_matrix(vertices));
}

fn create_distance_matrix(vertices: Vec<Vertex>) -> Vec<Vec<f64>> {
    let mut distance_matrix:Vec<Vec<f64>> = vec![vec![0.0; vertices.len()]; vertices.len()];
    for x in 0..vertices.len() {
        for y in 0..vertices.len() {
            distance_matrix[x][y] = dist(vertices[x], vertices[y]);
        }
    }
    return distance_matrix;
}

type Vertex = (f64, f64);

fn dist(a: Vertex, b: Vertex) -> f64 {
    let a_dist = (a.0 - b.0).abs();
    let b_dist = (a.1 - b.1).abs();
    return (a_dist.powi(2) + b_dist.powi(2)).sqrt();
}

fn held_karp(distances: Vec<Vec<f64>>) {
    // C contents:
    //   key:
    //     - visited nodes in this subset
    //     - the node (in this subset) we used to enter this subset
    //   value:
    //     - distance to this subset
    //     - the node previous to the one we ended here

    #![allow(non_snake_case)]
    let distance_len = distances.len();
    let mut C:HashMap<u64,Vec<f64>,BuildIdentityHasher> = HashMap::with_capacity_and_hasher(
        distance_len, 
        BuildHasherDefault::<IdentityHasher>::default());

    for i in 1..distance_len {
        C.entry(1 << (i - 1)).or_insert_with(|| vec![std::f64::MAX; distances.len()])[i] = distances[0][i];
    }

    for subset_size in 2..distance_len {
        println!("Subset size {}/{}", subset_size, distance_len);
        let subsets = (1..(distance_len)).combinations(subset_size);
        for subset in subsets {
            let mut current_subset = 0;
            for i in &subset {
                current_subset |= 1 << (i - 1);
            }
            let mut entries = vec![std::f64::MAX; distance_len];
            for node in &subset {
                let nod = *node;
                let prev_subset = current_subset ^ (1 << (nod - 1));

                let mut candidate_solution = std::f64::MAX;
                let mut prev_node = 0;

                for n in &C[&prev_subset] {
                    let prev_distance = n;
                    let new_distance = prev_distance + distances[prev_node][nod];
                    if new_distance < candidate_solution {
                        candidate_solution = new_distance;
                    }
                    prev_node += 1;
                }

                entries[nod] = candidate_solution;
            }
            C.insert(current_subset, entries);
        }
    }

    // find lowest distance that visits all nodes and then returns to starting position
    let mut tour_distances:Vec<f64> = C.iter()
     .filter(|(visited, _)| **visited == ((1 << distances.len() - 1) - 1) as u64)
     .flat_map(|(_, entries)| entries.iter().enumerate().map(|(i, entry)| entry + distances[0][i]))
     .collect();

    tour_distances.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", tour_distances.get(0));
}

#[derive(Debug, Clone, Copy, Default)]
struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!("IdentityHasher only supports u64 keys")
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

type BuildIdentityHasher = BuildHasherDefault<IdentityHasher>;

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_held_karp(b: &mut Bencher) {
        b.iter(|| held_karp(create_distance_matrix(vec![
            (3.433752748235324, 2.9215164273513206),
            (0.266027289402357, 3.367553812393056),
            (3.107592426409198, 3.091359997997841),
            (1.2770174634306963, 1.4543288785259425)
        ])));
    }
}