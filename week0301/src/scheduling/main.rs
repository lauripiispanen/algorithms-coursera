extern crate clap;
use clap::{Arg, App};

use common::read_file::read_numeric_lines;

fn main() {
    let matches = App::new("Greedy algorithm for scheduling")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();

    let jobs:Vec<(u64,u64)> = read_numeric_lines(input_file)
                                    .skip(1)
                                    .map(|e| (e[0],e[1]))
                                    .collect();

    println!("Weighted completion time: {}", schedule(jobs.clone(), |w, l| w as f64 - l as f64));
    println!("Weighted completion time: {}", schedule(jobs.clone(), |w, l| w as f64 / l as f64));
}

fn schedule<F>(mut jobs:Vec<(u64,u64)>, order: F) -> u64
where F: Fn(u64, u64) -> f64 {
    'outer: loop {
        let len = jobs.len() - 1;
        for mut i in 0..len {
            'inner: loop {
                let (job_1_weight, job_1_length) = jobs[i];
                let (job_2_weight, job_2_length) = jobs[i + 1];
                let diff_1 = order(job_1_weight, job_1_length);
                let diff_2 = order(job_2_weight, job_2_length);

                if diff_2 > diff_1 ||
                    (diff_1 == diff_2 && job_2_weight > job_1_weight) {
                    jobs.swap(i, i + 1);

                    if i <= 0 {
                        break 'inner;
                    }
                    i = i - 1;
                } else {
                    break 'inner;
                }
            }
        }
        break 'outer;
    }
    let (_, weighted_completion_time) = jobs.iter().fold((0, 0), |(time_elapsed, sum_w_compl), job| {
        let (job_weight, job_length) = job;
        let completion_time = time_elapsed + job_length;
        return (completion_time, sum_w_compl + (completion_time * job_weight));
    });

    return weighted_completion_time;
}
