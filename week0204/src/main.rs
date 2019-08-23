use std::env;
use std::fs::File;
use std::collections::{HashSet, VecDeque};
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: std::env::Args = env::args();
    if args.len() != 2 {
        panic!("Please provide name of data file as argument!");
    }
    let filename = args.last().unwrap();

    if !Path::new(&filename).is_file() {
        println!("ERROR: File {} does not exist", filename);
        std::process::exit(1);
    }

    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    let found = calculate_sum_2(file.lines().map(|x| x.unwrap().parse::<i64>().unwrap()), -10000, 10000);

    println!("found {}", found);
}

fn calculate_sum_2(n: impl Iterator<Item=i64>, from: i64, to: i64) -> usize {
    let mut all_nums: Vec<i64> = n.collect();

    all_nums.sort();
    let buf = VecDeque::from(all_nums);

    let mut left_pivot = 0;
    let mut right_pivot = buf.len() - 1;
    let mut found = HashSet::new();

    while left_pivot < right_pivot {
        let left = buf[left_pivot];
        let mut right = buf[right_pivot];
        let lower_bound = from - left;
        let upper_bound = to - left;
        while right > upper_bound {
            right_pivot -= 1;
            right = buf[right_pivot];
        }
        let mut index = right_pivot;
        while index > left_pivot && buf[index] >= lower_bound {
            let num = left + buf[index];
            if num >= from && num <= to && left != buf[index] {
                found.insert(num);
            }
            index -= 1;
        }

        left_pivot += 1;
    };
    return found.len();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sum_2() {
        let found = super::calculate_sum_2(vec![-3,-1,1,2,9,11,7,6,2].into_iter(), 3, 10);
        assert_eq!(8, found);
    }

    #[test]
    fn test_sum_2_2() {
        let found = super::calculate_sum_2(vec![-2,0,0,4].into_iter(), 0, 4);
        assert_eq!(2, found);
    }

    #[test]
    fn test_sum_2_3() {
        let found = super::calculate_sum_2(vec![-200000,0,0,400000].into_iter(), 0, 400000);
        assert_eq!(2, found);
    }

}