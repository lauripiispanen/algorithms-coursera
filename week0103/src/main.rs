use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("ERROR: pass in (only) the name of the file as argument.");
        std::process::exit(1);
    }
    let filename = args.last().unwrap();

    if !Path::new(filename).is_file() {
        println!("ERROR: File {} does not exist", filename);
        std::process::exit(1);
    }

    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    let nums:Vec<u64> = file.lines().map(|x| x.unwrap().parse::<u64>().unwrap()).collect();
    println!("Number of lines: {}", nums.len());
    println!("Number of comparisons when pivot == 0: {}", quick_sort(&mut nums.clone(), |_| 0));
    println!("Number of comparisons when pivot == len: {}", quick_sort(&mut nums.clone(), |x| x.len() - 1));
    println!("Number of comparisons when pivot == median-of-three: {}", quick_sort(&mut nums.clone(), median_of_three_pivot_idx));
}

fn median_of_three_pivot_idx(vec: &[u64]) -> usize {
    let first = (vec[0], 0);
    let mid_idx = if vec.len() % 2 == 0 {
        (vec.len() / 2) - 1
    } else {
        vec.len() / 2
    };
    let middle = (vec[mid_idx], mid_idx);
    (vec[(vec.len() - 1) / 2], (vec.len() - 1) / 2);
    let last = (vec[vec.len() - 1], vec.len() - 1);
    let mut nums = vec![first, middle, last];

    nums.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    return nums[1].1;
}

fn quick_sort(vec: &mut [u64], pivot_idx_fn: fn(&[u64]) -> usize) -> u64 {
    let len = vec.len();
    if len < 2 {
        return 0;
    }
    let pivot_idx = pivot_idx_fn(vec);
    if pivot_idx > 0 {
        vec.swap(0, pivot_idx);
    }
    let pivot = vec[0];
    let mut _i = 1;
    for _j in 1..len {
        if vec[_j] < pivot {
            vec.swap(_j, _i);
            _i += 1;
        }
    }
    vec.swap(0, _i - 1);
    let num_a = quick_sort(&mut vec[0..(_i - 1)], pivot_idx_fn);
    let num_b = quick_sort(&mut vec[_i..len], pivot_idx_fn);
    return num_a + num_b + len as u64 - 1;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_quick_sort() {

        let mut v = [1];
        super::quick_sort(&mut v, |_| 0);
        assert_eq!(v, [1]);

        let mut v = [1,5,8,2,4,3,7];
        super::quick_sort(&mut v, |_| 0);
        assert_eq!(v, [1,2,3,4,5,7,8]);

        let mut v = [1,5,8,2,4,3,7];
        super::quick_sort(&mut v, |x| x.len() - 1);
        assert_eq!(v, [1,2,3,4,5,7,8]);


        let mut v = [100,5,82,2,4,3,7];
        super::quick_sort(&mut v, |x| x.len() - 1);
        assert_eq!(v, [2,3,4,5,7,82,100]);
    }

    #[test]
    fn median_of_three_pivot_idx() {
        assert_eq!(super::median_of_three_pivot_idx(&[8,2,4,5,7,1]), 2);
        assert_eq!(super::median_of_three_pivot_idx(&[4,5,6,7]), 1);

        assert_eq!(super::median_of_three_pivot_idx(&[8,2,4,5,7,1]), 2);
        assert_eq!(super::median_of_three_pivot_idx(&[5,4,6,7]), 0);
        assert_eq!(super::median_of_three_pivot_idx(&[7,4,6,5]), 3);
    }

}