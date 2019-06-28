use std::env;

mod stringmath;

fn main() {
    let args = take_exactly_args(2);
    let first = &args[0];
    let second = &args[1];

    ensure_numeric(first);
    ensure_numeric(second);
    println!("{}", karatsuba_multiply(first, second));
}

fn karatsuba_multiply(_first: &str, _second: &str) -> String {
    if _first.len() == 1 || _second.len() == 1 {
        if _first.len() < _second.len() {
            return stringmath::multiply(_first, _second);
        } else {
            return stringmath::multiply(_second, _first);
        }
    } else {
        let len = std::cmp::min(_first.len(), _second.len());
        let _pow = len - 1;

        let (first_1, first_2) = _first.split_at(_first.len() - _pow);
        let (second_1, second_2) = _second.split_at(_second.len() - _pow);;

        let z0 = karatsuba_multiply(first_2, second_2);
        let z2 = karatsuba_multiply(first_1, second_1);
        let k = &karatsuba_multiply(
            &stringmath::add(first_1, first_2),
            &stringmath::add(second_1, second_2)
        );
        let z1 = &stringmath::subtract(
            &stringmath::subtract(
                k, &z2),
                &z0
            );

        let res = stringmath::add(
            &stringmath::add(
                &stringmath::pow_10(&z2, _pow * 2),
                &stringmath::pow_10(&z1, _pow)
            ),
            &z0
        );
        return res;
    }
}

fn take_exactly_args(num: usize) -> Vec<String> {
let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len != num + 1 {
        std::process::exit(1);
    }
    
    return vec![args[args_len - 2].clone(), args[args_len - 1].clone()];
}

fn ensure_numeric(s: &String) {
    s.chars().map(|x| x.to_digit(10)).filter(|x| x.is_none()).for_each(|_| std::process::exit(2));
}