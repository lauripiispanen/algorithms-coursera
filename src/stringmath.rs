pub fn max<'a>(_first: &'a str, _second: &'a str) -> &'a str {
    let negative_first = is_negative(_first);
    let negative_second = is_negative(_second);
    if negative_first != negative_second {
        if negative_first {
            return _second;
        } else {
            return _first;
        }
    } else {
        let max_len = _first.len().max(_second.len());
        let v1 = std::iter::repeat(0)
                    .take(max_len - _first.len())
                    .chain(
                        _first
                            .chars()
                            .filter_map(|x| x.to_digit(10)))
                    .take(max_len);

        let v2 = std::iter::repeat(0)
                    .take(max_len - _second.len())
                    .chain(
                        _second
                            .chars()
                            .filter_map(|x| x.to_digit(10)))
                    .take(max_len);
        

        let first_diff = v1.zip(v2)
                .find_map(|(x, y)| if x > y {
                        if negative_first {
                            Some(_second)
                        } else {
                            Some(_first)
                        }
                    } else if y > x {
                        if negative_first {
                            Some(_first)
                        } else {
                            Some(_second)
                        }
                    } else {
                        None
                    });

        return match first_diff {
              Some(x) => x,
              None => _first
        }
    }
}

pub fn multiply(_first: &str, _second: &str) -> String {
    let _first_num = _first.parse::<u128>().unwrap();
    let _second_num = _second.parse::<u128>().unwrap();
    let mut ret:u128 = 0;
    for _ in 0.._first_num {
        ret += _second_num;
    }
    return ret.to_string();
}

fn is_negative(_num: &str) -> bool {
    return _num.chars().next() == Some('-');
}

pub fn add(_first: &str, _second: &str) -> String {
    let negative_first = is_negative(_first);
    let negative_second = is_negative(_second);

    if negative_first == negative_second {
        // same sign, do regular add and then plop the correct sign back in
        let mut max_len = _first.len().max(_second.len());
        if negative_first {
            max_len -= 1;
        }

        let v1 = _first.chars()
                        .rev()
                        .filter_map(|x| x.to_digit(10))
                        .chain(std::iter::repeat(0))
                        .take(max_len);

        let v2 = _second.chars()
                        .rev()
                        .filter_map(|x| x.to_digit(10))
                        .chain(std::iter::repeat(0))
                        .take(max_len);

        let mut chain = v1.zip(v2).map(|(x, y)| x + y);

        let mut ret = Vec::<char>::with_capacity(max_len);
        let mut carry = false;
        while let Some(x) = chain.next() {
            let real_x = if carry {
                x + 1
            } else {
                x
            };

            if real_x >= 10 {
                carry = true;
                ret.push(std::char::from_digit(real_x - 10, 10).unwrap());
            } else {
                carry = false;
                ret.push(std::char::from_digit(real_x, 10).unwrap());
            }
        }
        if carry {
            ret.push('1');
        }
        let mut past_zeros = false;
        let mut r = ret.into_iter().rev().filter(|x| {
            if !past_zeros && x != &'0' {
                past_zeros = true;
            }
            return past_zeros;
        }).collect::<String>();

        if r.len() == 0 {
            return String::from("0");
        }

        if negative_first {
            r.insert_str(0, "-");
        }
        return r;

    } else {
        // different sign, take abs from both values, subtract, then plop in sign of higher number
        let _first_abs = if negative_first {
            &_first[1.._first.len()]
        } else {
            _first
        };

        let _second_abs = if negative_second {
            &_second[1.._second.len()]
        } else {
            &_second
        };

        let max_len = _first_abs.len().max(_second_abs.len());

        let (_f, _s) = if max(_first_abs, _second_abs) == _first_abs {
            (_first_abs, _second_abs)
        } else {
            (_second_abs, _first_abs)
        };

        let v1 = _f.chars()
                    .rev()
                    .filter_map(|x| x.to_digit(10))
                    .chain(std::iter::repeat(0))
                    .take(max_len);
        let v2 = _s.chars()
                    .rev()
                    .filter_map(|x| x.to_digit(10))
                    .chain(std::iter::repeat(0))
                    .take(max_len);
        
        let mut chain = v1.zip(v2);
        let mut ret = Vec::<char>::with_capacity(max_len);
        let mut carry = false;
        while let Some((x, y)) = chain.next() {
            let mut real_y = y;
            if carry {
                real_y = real_y + 1;
            }
            if real_y > x {
                carry = true;
                ret.push(std::char::from_digit((10 + x) - real_y, 10).unwrap());
            } else {
                ret.push(std::char::from_digit(x - real_y, 10).unwrap());
                carry = false;
            }
        }

        let mut past_zeros = false;
        let mut r = ret.into_iter().rev().filter(|x| {
            if !past_zeros && x != &'0' {
                past_zeros = true;
            }
            return past_zeros;
        }).collect::<String>();

        if _f == _first_abs && negative_first ||
           _f == _second_abs && negative_second {
            r.insert_str(0, "-");
        }
        return r;
    }
}

pub fn subtract(_first: &str, _second: &str) -> String {
    // flip the second symbol sign and then do an add
    let negative_second = is_negative(_second);
    if negative_second {
        return add(_first, &_second[1.._second.len()]);
    } else {
        let mut _s = String::with_capacity(1 + _second.len());
        _s.push('-');
        _s.push_str(_second);
        return add(_first, &_s);
    }
}

pub fn pow_10(_num: &str, _pow: usize) -> String {
    let mut _s = String::with_capacity(1 + _pow);
    _s.push_str(_num);
    for _ in 0.._pow {
        _s.push('0');
    }
    return _s;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_stringmath_pow_10() {
        assert_eq!(super::pow_10("2", 2), "200");
    }

    #[test]
    fn test_stringmath_add() {
        assert_eq!(super::add("2", "3"), "5");
        assert_eq!(super::add("17", "18"), "35");
        assert_eq!(super::add("22", "3"), "25");
        assert_eq!(super::add("8", "9"), "17");
        assert_eq!(super::add("2", "1243"), "1245");

        assert_eq!(super::add("-2", "-6"), "-8");
        assert_eq!(super::add("-20", "-6"), "-26");

        assert_eq!(super::add("20", "-6"), "14");
        assert_eq!(super::add("-6", "20"), "14");
        assert_eq!(super::add("-6", "2"), "-4");
        assert_eq!(super::add("3960", "45"), "4005");

        assert_eq!(super::add("000", "45"), "45");
        assert_eq!(super::add("0", "0"), "0");
    }
    
    #[test]
    fn test_stringmath_max() {
        assert_eq!(super::max("2", "-3"), "2");
        assert_eq!(super::max("-2", "3"), "3");
        assert_eq!(super::max("2", "7"), "7");
        assert_eq!(super::max("22", "17051"), "17051");
        assert_eq!(super::max("-22", "-17051"), "-22");
    }
    
    #[test]
    fn test_stringmath_multiply() {
        assert_eq!(super::multiply("0", "45"), "0");
        assert_eq!(super::multiply("0000", "45"), "0");
    }

    #[test]
    fn test_stringmath_subtract() {
        assert_eq!(super::subtract("5", "3"), "2");
        assert_eq!(super::subtract("25", "3"), "22");
        assert_eq!(super::subtract("1245", "1243"), "2");

        assert_eq!(super::subtract("-8", "-6"), "-2");
        assert_eq!(super::subtract("8", "9"), "-1");
        assert_eq!(super::subtract("-26", "-6"), "-20");

        assert_eq!(super::subtract("14", "-6"), "20");
        assert_eq!(super::subtract("14", "20"), "-6");
        assert_eq!(super::subtract("-4", "2"), "-6");
    }
}