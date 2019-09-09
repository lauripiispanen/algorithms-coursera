use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn read_numeric_lines<T>(input_file: &str) -> impl Iterator<Item = Vec<T>>
where T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    let f = File::open(input_file).unwrap();
    let file = BufReader::new(f);

    return file.lines()
                .map(|x| {
                    return x.unwrap()
                            .split_whitespace()
                            .map(|x| x.parse::<T>().unwrap())
                            .collect();
                });
}