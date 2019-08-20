use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

mod binheap;

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

    let nums = file.lines().map(|x| x.unwrap().parse::<u64>().unwrap());

    let (median, median_sum) = calculate_median(nums);

    println!("median: {}", median.unwrap());
    println!("median sum: {}", median_sum % 10000);
}

fn calculate_median(nums: impl Iterator<Item = u64>) -> (Option<u64>, u64) {
    let mut right_heap = binheap::BinHeap::<u64>::new_minheap();
    let mut left_heap = binheap::BinHeap::<u64>::new_maxheap();
    let mut median_sum = 0;
    let mut median:Option<u64> = None;

    for n in nums {
        match median {
            None => {
                right_heap.insert(n);
            },
            Some(m) => {
                if n < m {
                    left_heap.insert(n);
                } else {
                    right_heap.insert(n);
                }
            }
        }

        while right_heap.len() > left_heap.len() + 1 {
            left_heap.insert(right_heap.extract().unwrap());
        }
        while left_heap.len() > right_heap.len() + 1 {
            right_heap.insert(left_heap.extract().unwrap());
        }

        if right_heap.len() > left_heap.len() {
            median = right_heap.peek().map(|y| *y);
        } else {
            median = left_heap.peek().map(|y| *y);
        }

        median_sum += median.unwrap();
    }

    return (median, median_sum);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_median() {
        let (median, median_sum) = super::calculate_median(vec![2,4].into_iter());
        assert_eq!(2, median.unwrap());
        assert_eq!(4, median_sum);
    }

    #[test]
    fn test_median2() {
        let (median, median_sum) = super::calculate_median(vec![1,2,2,6,8].into_iter());
        assert_eq!(2, median.unwrap());
        assert_eq!(8, median_sum);
    }

    #[test]
    fn test_median3() {
        let (median, median_sum) = super::calculate_median(vec![10,2,1,6,2,8].into_iter());
        assert_eq!(2, median.unwrap());
        assert_eq!(20, median_sum);
    }

    #[test]
    fn test_median4() {
        let (median, median_sum) = super::calculate_median(vec![8,10,1,2].into_iter());
        assert_eq!(2, median.unwrap());
        assert_eq!(26, median_sum);
    }

    #[test]
    fn test_median5() {
        let (median, median_sum) = super::calculate_median(vec![8,10,11,1,2].into_iter());
        assert_eq!(8, median.unwrap());
        assert_eq!(42, median_sum);
    }

    #[test]
    fn test_median6() {
        let (_, median_sum) = super::calculate_median(vec![1,666,10,667,100,2,3].into_iter());
        assert_eq!(142, median_sum);
    }

    #[test]
    fn test_median7() {
        let (_, median_sum) = super::calculate_median(vec![6331,2793,1640,9290,225,625,6195,2303,5685,1354].into_iter());
        assert_eq!(9335, median_sum % 10000);
    }

    #[test]
    fn test_median8() {
        let (median, median_sum) = super::calculate_median(vec![23,9,35,4,13,24,2,5,27,1,34,8,15,39,32,22,29,21,19,20,36,33,7,31,14,17,26,16,38,6,30,40,25,28,11,37,3,10,18,12].into_iter());
        assert_eq!(20, median.unwrap());
        assert_eq!(717, median_sum % 10000);
    }
}