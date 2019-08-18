use std::env;
use std::fs::File;
use std::path::Path;

use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

use std::convert::TryInto;
use std::cmp::Ordering;

mod binheap;

struct Edge {
    from: usize,
    to: usize,
    length: u32
}

#[derive(Eq)]
struct Node {
    id: usize,
    distance: u64
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("ERROR: pass in the name of the file and target nodes (comma-separated) as argument.");
        std::process::exit(1);
    }

    let filename = &args[1];

    if !Path::new(filename).is_file() {
        println!("ERROR: File {} does not exist", filename);
        std::process::exit(1);
    }

    let target_nodes = &args[2].split(",").map(|n| n.parse::<usize>().unwrap());

    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    let nodes:Vec<(usize, Vec<Edge>)>= file.lines()
        .map(|r| r.unwrap())
        .map(|r| r.split_whitespace().map(|s| s.to_string()).collect())
        .map(|r:Vec<String>| {
            let mut row_entries = r.into_iter(); 
            let from = row_entries.next().unwrap().parse::<usize>().unwrap();

            let siblings = row_entries.map(move |e:String| {
                let v:Vec<&str> = e.split(",").take(2).collect();
                if let [to, distance] = &v[..] {
                    return Edge {
                        from: from,
                        to: to.parse::<usize>().unwrap(),
                        length: distance.parse::<u32>().unwrap()
                    };
                } else {
                    panic!("Unexpected file format!");
                }
            }).collect();

            return (from, siblings);
        }).collect();

    println!("We have {} nodes", nodes.len());
    let last_node = nodes.last().and_then(|(from, _)| (*from).try_into().ok()).unwrap();

    let mut visited = HashSet::<usize>::new();
    let mut distances = vec![1000000; last_node];
    let mut current_node = 0;
    distances[0] = 0;

    let mut neighbors = binheap::BinHeap::<Node>::new_minheap();

    while visited.len() < nodes.len() {
        for n in &nodes[current_node].1 {
            let node_id = n.to - 1;
            if !visited.contains(&node_id) {
                let dist = distances[current_node] + n.length as u64;
                if dist < distances[node_id] {
                    distances[node_id] = dist;
                }
                neighbors.insert(Node {
                    id: node_id,
                    distance: dist
                });
            }
        };

        let next = neighbors.extract().unwrap();
        visited.insert(next.id);
        current_node = next.id
    }

    let target_distances = target_nodes.to_owned()
                                       .map(|n| distances[n - 1].to_string())
                                       .collect::<Vec<String>>()
                                       .join(",");

    println!("Distance to nodes: {}", target_distances);
}