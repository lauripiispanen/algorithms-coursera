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

    let mut i = read_file::read_numeric_lines::<f64>(input_file);

    let _:usize = i.next().map(|v| v[0] as usize).unwrap();

    let mut vertices:Vec<Vertex> = i.map(|v| (v[1], v[2])).collect();

    let mut prev:Option<Vertex> = None;
    let mut first:Option<Vertex> = None;
    let mut dist:f64 = 0.0;
    while vertices.len() > 0 {
        match prev {
            Some(v) => {
                let mut next_dist = std::f64::MAX;
                let mut next_i = std::usize::MAX;

                for (i, n) in vertices.iter().enumerate() {
                    let d = dist_euql(v, *n);
                    if d < next_dist {
                        next_dist = d;
                        next_i = i;
                    }
                }
                dist += next_dist;
                prev = Some(vertices.remove(next_i));
            },
            None => {
                prev = Some(vertices.remove(0));
                first = prev;
            }
        }
    }

    dist += dist_euql(prev.unwrap(), first.unwrap());

    println!("{}", dist.floor());
}

type Vertex = (f64, f64);

fn dist_euql(a: Vertex, b: Vertex) -> f64 {
    return ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
}