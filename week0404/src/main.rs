mod read_file;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let arg_matches = App::new("TSP with heuristics")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();

    let mut i = read_file::read_numeric_lines::<i64>(input_file);

    let max:usize = i.next().map(|v| v[0] as usize).unwrap();
    let edges:Vec<(i64, i64)> = i.map(|v| (v[0], v[1])).collect();
    let (g, gt) = to_graph(edges, max);
    println!("{}", solve_2_sat(g, gt));
}

fn to_graph(edges: Vec<(i64, i64)>, max: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    // In the graphs vertices with indices 2k and 2k+1 are the two vertices corresponding to variable k with 2k+1 corresponding to the negated variable.
    let mut g:Vec<Vec<usize>> = vec![vec![]; max * 2];
    let mut gt:Vec<Vec<usize>> = vec![vec![]; max * 2];
    for (a, b) in edges {
        let mut a_i = indexify(a * -1);
        let mut b_i = indexify(b);
        g[a_i].push(b_i);
        gt[b_i].push(a_i);

        a_i = indexify(a);
        b_i = indexify(b * -1);
        g[b_i].push(a_i);
        gt[a_i].push(b_i);
    }
    return (g, gt);
}

fn indexify(a: i64) -> usize {
    return match a {
        i if i < 0 => ((i.abs() - 1) * 2 + 1) as usize,
        i => ((i - 1) * 2) as usize
    };
}

fn solve_2_sat(g: Vec<Vec<usize>>, gt: Vec<Vec<usize>>) -> bool {
    let n:usize = g.len();
    let mut used = vec![false; n];
    let mut order = vec![];

    fn dfs1(v: usize, g: &Vec<Vec<usize>>, used: &mut Vec<bool>, order: &mut Vec<usize>) {
        if used[v] {
            return;
        }
        used[v] = true;
        for u in g[v].iter() {
            if !used[*u] {
                dfs1(*u, g, used, order);
            }
        }
        order.push(v);
    }

    for i in 0..n {
        if !used[i] {
            dfs1(i, &g, &mut used, &mut order);
        }
    }

    fn dfs2(v: usize, gt: &Vec<Vec<usize>>, cl: usize, comp: &mut Vec<i64>) {
        comp[v] = cl as i64;
        for u in gt[v].iter() {
            if comp[*u] == -1 {
                dfs2(*u, gt, cl, comp);
            }
        }
    }

    let mut comp = vec![-1; n];
    let mut j = 0;
    for i in 0..n {
        let v = order[n - i - 1];
        if comp[v] == -1 {
            dfs2(v, &gt, j, &mut comp);
            j += 1;
        }
    }

    for i in (0..n).step_by(2) {
        if comp[i] == comp[i + 1] {
            return false;
        }
    }
    return true;
}