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
    println!("Number of inversions: {}", count_inversions_and_sort(&nums).0);
}

fn count_inversions_and_sort(v: &[u64]) -> (u64, Vec<u64>) {
    if v.len() < 1 {
        return (0, Vec::new());
    } else if v.len() == 1 {
        return (0, Vec::from(v));
    } else {
        let split_point = v.len() / 2;
        let (x, a) = count_inversions_and_sort(&v[0..split_point]);
        let (y, b) = count_inversions_and_sort(&v[split_point..v.len()]);
        let (z, c) = count_split_inversions(&a, &b);
        return ((x + y + z), c);
    }
}

fn count_split_inversions(a: &[u64], b: &[u64]) -> (u64, Vec<u64>) {
    let a_len = a.len();
    let b_len = b.len();
    let l = a_len + b_len;
    let mut a_i = 0;
    let mut b_i = 0;
    let mut inv = 0;
    let mut ret = Vec::with_capacity(l);
    for _ in 0..l {
        if a_i >= a_len {
            ret.push(b[b_i]);
            b_i += 1;
        } else if b_i >= b.len() {
            ret.push(a[a_i]);
            a_i += 1;
        } else if a[a_i] <= b[b_i] {
            ret.push(a[a_i]);
            a_i += 1;
        } else {
            ret.push(b[b_i]);
            inv += a_len - a_i;
            b_i += 1;
        }
    }
    return (inv as u64, ret);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_count_inversions_and_sort() {
        assert_eq!(super::count_inversions_and_sort(&[1, 3, 5, 2, 4, 6]).0, 3);
        assert_eq!(super::count_inversions_and_sort(&[1, 5, 3, 2, 4]).0, 4);
        assert_eq!(super::count_inversions_and_sort(&[1, 3, 5, 2, 4, 6, 7]).0, 3);
        assert_eq!(super::count_inversions_and_sort(&[1, 1, 1]).0, 0);
        assert_eq!(super::count_inversions_and_sort(&[1, 3, 2, 4, 5, 6]).0, 1);
        assert_eq!(super::count_inversions_and_sort(&[1, 3, 4, 2, 5, 6]).0, 2);
        assert_eq!(super::count_inversions_and_sort(&[1, 3, 4, 2, 5, 6]).0, 2);
        assert_eq!(super::count_inversions_and_sort(&[1]).0, 0);
        assert_eq!(super::count_inversions_and_sort(&[1, 2]).0, 0);
        assert_eq!(super::count_inversions_and_sort(&[2, 1]).0, 1);
        assert_eq!(super::count_inversions_and_sort(&[1, 3, 2, 4, 5, 61252]).0, 1);
        assert_eq!(super::count_inversions_and_sort(&[1, 3, 5, 2, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14]).0, 3);
        assert_eq!(super::count_inversions_and_sort(&[4, 80, 70, 23, 9, 60, 68, 27, 66, 78, 12, 40, 52, 53, 44, 8, 49, 28, 18, 46, 21, 39, 51, 7, 87, 99, 69, 62, 84, 6, 79, 67, 14, 98, 83, 0, 96, 5, 82, 10, 26, 48, 3, 2, 15, 92, 11, 55, 63, 97, 43, 45, 81, 42, 95, 20, 25, 74, 24, 72, 91, 35, 86, 19, 75, 58, 71, 47, 76, 59, 64, 93, 17, 50, 56, 94, 90, 89, 32, 37, 34, 65, 1, 73, 41, 36, 57, 77, 30, 22, 13, 29, 38, 16, 88, 61, 31, 85, 33, 54]).0, 2372);
        assert_eq!(super::count_inversions_and_sort(&[37, 7, 2, 14, 35, 47, 10, 24, 44, 17, 34, 11, 16, 48, 1, 39, 6, 33, 43, 26, 40, 4, 28, 5, 38, 41, 42, 12, 13, 21, 29, 18, 3, 19, 0, 32, 46, 27, 31, 25, 15, 36, 20, 8, 9, 49, 22, 23, 30, 45]).0, 590);
    }
}