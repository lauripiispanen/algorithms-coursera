extern crate clap;
use clap::{Arg, App};

use common::read_file::read_numeric_lines;
use common::union_find::UnionFind;

fn main() {
    let arg_matches = App::new("Clustering algorithm that outputs the max spacing of k-clusters")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();

    let mut i = read_numeric_lines::<usize>(input_file);
    let node_count = i.next().unwrap()[0];

    let edges:Vec<(usize, usize, usize)> = i.map(|r|
        (r[0], r[1], r[2])
    ).collect();

    let max_spacing = calculate_max_spacing(node_count, edges, 4);

    println!("{}", max_spacing);
}

fn calculate_max_spacing(node_count: usize, mut edges: Vec<(usize, usize, usize)>, max_clusters: usize) -> usize {
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut u = UnionFind::new(node_count);
    let mut max_spacing = 0;
    'main: for (a, b, cost) in edges {
        u.union(a - 1, b - 1);
        if u.num_sets() < max_clusters {
            max_spacing = cost;
            break 'main;
        }
    }

    return max_spacing;
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_cluster() {
        let max_spacing = super::calculate_max_spacing(5,
        vec![
            (1,2,1),
            (1,3,100),
            (1,4,100),
            (1,5,100),
            (2,3,100),
            (2,4,100),
            (2,5,100),
            (3,4,10),
            (3,5,10),
            (4,5,10)
        ], 2);

        assert_eq!(100, max_spacing);
    }

    #[test]
    fn test_cluster2() {
        let max_spacing = super::calculate_max_spacing(8,
        vec![
            (1,2,50),
            (1,3,5),
            (1,4,8),
            (1,5,47),
            (1,6,3),
            (1,7,42),
            (1,8,36),
            (2,3,60),
            (2,4,34),
            (2,5,6),
            (2,6,27),
            (2,7,62),
            (2,8,61),
            (3,4,58),
            (3,5,53),
            (3,6,37),
            (3,7,54),
            (3,8,12),
            (4,5,63),
            (4,6,29),
            (4,7,52),
            (4,8,44),
            (5,6,1),
            (5,7,16),
            (5,8,6),
            (6,7,45),
            (6,8,52),
            (7,8,60)
        ], 4);

        assert_eq!(6, max_spacing);
    }

}