mod read_file;
mod binheap;

extern crate clap;
use clap::{Arg, App};
use std::convert::TryInto;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let arg_matches = App::new("shortest shortest path coding example")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();

    let mut i = read_file::read_numeric_lines::<i32>(input_file);
    let num_vertices:usize = i.next().map(|v| v[0].try_into().unwrap()).unwrap();
    let n_vertices = num_vertices.try_into().unwrap();

    let mut edges:Vec<Edge> = i.map(|v| (v[0] - 1, v[1] - 1, v[2] as i128)).collect();
    let mut additional_edges:Vec<Edge> = (0..n_vertices).map(|i| 
        (n_vertices, i, 0)
    ).collect();

    println!("total number of vertices {}", n_vertices);
    
    edges.append(&mut additional_edges);

    let (distances, _) = bellman_ford(num_vertices + 1, &edges, n_vertices);
    edges = edges.into_iter()
                .filter(|(u, _, _)| *u != n_vertices)
                .collect();

    let reweighted_edges:Vec<Edge> = edges.iter().map(|(u, v, w)| {
                                            let idx_u:usize = (*u).try_into().unwrap();
                                            let idx_v:usize = (*v).try_into().unwrap();
                                            (*u, *v, (*w as i128) + distances[idx_u] - distances[idx_v])
                                        })
                                        .collect();

    let mut min_cost = std::i128::MAX;

    for i in 0..num_vertices {
        let d_distances = dijkstra(num_vertices, &reweighted_edges, i);
        for (d_index, node_opt) in d_distances.iter().enumerate() {
            match node_opt {
                Some(_) => {
                    // backtrace to get the original path
                    let mut p_index = d_index;
                    let mut total_cost = 0;
                    while let Some(p_edge_idx) = d_distances[p_index].clone().and_then(|n| n.preceding_edge) {
                        let edge = edges[p_edge_idx];
                        total_cost += edge.2;
                        p_index = edge.0.try_into().unwrap();
                    }
                    if total_cost < min_cost {
                        min_cost = total_cost;
                    }
                },
                None => {}
            }
        }
    }
    println!("{}", min_cost);
}

type Edge = (i32, i32, i128);

fn bellman_ford(num_vertices: usize, edges: &Vec<Edge>, source: i32) -> (Distances, Predecessors) {
    let mut predecessor:Predecessors = vec![None; num_vertices];
    let mut distance:Distances = vec![std::i128::MAX; num_vertices];

    let source_i:usize = source.try_into().unwrap();

    distance[source_i] = 0;

    for _ in 1..num_vertices {
        edges.iter().for_each(|(u, v, w)| {
            let idx_u:usize = (*u).try_into().unwrap();
            let idx_v:usize = (*v).try_into().unwrap();
            if (distance[idx_u] < std::i128::MAX) && 
               (distance[idx_u] + *w as i128) < distance[idx_v] {
                distance[idx_v] = distance[idx_u] + *w as i128;
                predecessor[idx_v] = Some((*u).try_into().unwrap());
            }
        })
    }
    
    edges.iter().for_each(|(u, v, w)| {
        let idx_u:usize = (*u).try_into().unwrap();
        let idx_v:usize = (*v).try_into().unwrap();
        if (distance[idx_u] + *w as i128) < distance[idx_v] {
            panic!("Graph contains a negative cycle!");
        }
    });

    return (distance, predecessor);
}

type Predecessors = Vec<Option<usize>>;
type Distances = Vec<i128>;

fn dijkstra(num_vertices: usize, edges: &Vec<Edge>, source: usize) -> DijkstraDistances {
    let neighbors = edges.iter().enumerate().fold(HashMap::new(), |mut m, (edge_idx, (u, v, w))| {
        let idx_u:usize = (*u).try_into().unwrap();
        m.entry(idx_u).or_insert(Vec::new()).push(edge_idx);
        return m;
    });
    
    let mut distances:Vec<Option<DijkstraNode>> = vec![None; num_vertices];

    distances[source] = Some(DijkstraNode {
        distance: 0,
        preceding_edge: None
    });

    let mut q = binheap::BinHeap::<Node>::new_minheap();
    q.insert(Node {
        u: source,
        distance: 0
    });

    let mut visited = HashSet::new();

    while let Some(Node { u, distance }) = q.extract() {
        if !visited.insert(u) {
            continue;
        }

        if let Some(neighbor_list) = neighbors.get(&u) {
            for edge_idx in neighbor_list {
                let (_, v, w) = edges[*edge_idx];

                let idx_v:usize = v.try_into().unwrap();
                let new_distance = distance + w;
                let new_is_smaller = &distances[idx_v].clone().map(|d| new_distance < d.distance).unwrap_or(true);
                if *new_is_smaller {
                    distances[idx_v] = Some(DijkstraNode {
                        distance: new_distance,
                        preceding_edge: Some(*edge_idx)
                    });
                    q.insert(Node {
                        u: idx_v,
                        distance: new_distance
                    })
                }
            }
        }
    }

    return distances;
}

#[derive(Clone)]
#[derive(Debug)]
struct DijkstraNode {
    distance: i128,
    preceding_edge: Option<usize>
}

type DijkstraDistances = Vec<Option<DijkstraNode>>;

#[derive(Eq)]
struct Node {
    u: usize,
    distance: i128
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
