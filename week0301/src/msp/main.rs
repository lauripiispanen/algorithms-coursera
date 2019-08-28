extern crate clap;
use clap::{Arg, App};

use common::read_file::read_numeric_lines;
use common::binheap::BinHeap;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Eq)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64
}

impl Edge {
    fn flip(&self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
            cost: self.cost
        }
    }

    fn from(t: Vec<i64>) -> Edge {
        Edge {
            from: t[0] as usize,
            to: t[1] as usize,
            cost: t[2]
        }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn main() {
    let matches = App::new("Greedy algorithm for determining the minimum spanning tree of a graph")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();

    let nodes = read_numeric_lines::<i64>(input_file)
                                    .map(Edge::from);
    
    let cost = calculate_msp_cost(nodes);
    println!("MSP cost: {}", cost);
}

fn calculate_msp_cost<X>(node_list:X) -> i64
where X: Iterator<Item = Edge> {
    let nodes:HashMap<usize, Vec<Edge>> = node_list.fold(HashMap::new(), |mut m, e| {
                                        let flipped = e.flip();
                                        m.entry(e.from).or_insert(Vec::new()).push(e);
                                        m.entry(flipped.from).or_insert(Vec::new()).push(flipped);
                                        return m;
                                    });
    let mut node = *nodes.keys().next().unwrap();
    let mut min_heap = BinHeap::new_minheap();
    let mut visited = HashSet::new();
    visited.insert(node);
    let mut cost = 0;
    let empty = vec![];
    'outer: while visited.len() != nodes.len() {
        let neighbors = nodes.get(&node).unwrap_or(&empty).iter().filter(|e| !visited.contains(&e.to));
        for neighbor in neighbors {
            min_heap.insert(neighbor);
        }
        let mut min_neighbor = min_heap.extract();
        while min_neighbor.filter(|e| !visited.contains(&e.to)).is_none() {
            if min_heap.len() <= 0 {
                break 'outer;
            }
            min_neighbor = min_heap.extract();
        }
        match min_neighbor {
            Some(edge) => {
                node = edge.to;
                visited.insert(node);
                cost += edge.cost;
            },
            None => {
                // we've run out of potential neighbors!
                break 'outer;
            }
        }
    }
    return cost;
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_msp() {
        let cost = super::calculate_msp_cost(vec![
            vec![1,2,1],
            vec![2,4,2],
            vec![3,1,4],
            vec![4,3,5],
            vec![4,1,3]
        ].into_iter().map(super::Edge::from));

        assert_eq!(7, cost);
    }

    #[test]
    fn test_msp2() {
        let cost = super::calculate_msp_cost(vec![
            vec![1, 2, 1],
            vec![1, 8, 8],
            vec![2, 8, 2],
            vec![8, 3, 6],
            vec![2, 3, 9],
            vec![8, 4, 9],
            vec![4, 3, 4],
            vec![8, 7, 2],
            vec![4, 7, 1],
            vec![4, 5, 2],
            vec![3, 5, 5],
            vec![7, 5, 7],
            vec![7, 6, 10],
            vec![5, 6, 3]
        ].into_iter().map(super::Edge::from));

        assert_eq!(15, cost);
    }

    #[test]
    fn test_msp3() {
        let cost = super::calculate_msp_cost(vec![
            vec![1, 2, 6],
            vec![1, 4, 5],
            vec![1, 5, 4],
            vec![2, 4, 1],
            vec![2, 5, 2],
            vec![2, 3, 5],
            vec![2, 6, 3],
            vec![3, 6, 4],
            vec![4, 5, 2],
            vec![5, 6, 4]
        ].into_iter().map(super::Edge::from));

        assert_eq!(14, cost);
    }

}