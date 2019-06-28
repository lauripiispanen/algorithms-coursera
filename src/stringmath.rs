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
    let _first_num = _first.parse::<u32>().unwrap();
    let _second_num = _second.parse::<u32>().unwrap();
    let mut ret = 0;
    for _ in 0.._first_num {
        ret += _second_num;
    }
    return ret.to_string();
}

fn is_negative(_num: &str) -> bool {
    return _num.chars().next() == Some('-');
}

fn read_from_end_and_pad<'a>(_str: &'a str, up_to: usize) -> Box<std::iter::Iterator<Item = u32>> {
    return Box::new(_str
                .chars()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .filter_map(|x| x.to_digit(10))
                .chain(std::iter::repeat(0))
                .take(up_to))
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

        let v1 = read_from_end_and_pad(_first, max_len);
        let v2 = read_from_end_and_pad(_second, max_len);

        let mut chain = v1.zip(v2).map(|(x, y)| x + y);

        let mut ret = String::from("");
        let mut carry = false;
        while let Some(x) = chain.next() {
            let real_x = if carry {
                x + 1
            } else {
                x
            };

            if real_x >= 10 {
                carry = true;
                ret.insert_str(0, &(real_x - 10).to_string());
            } else {
                carry = false;
                ret.insert_str(0, &real_x.to_string());
            }
        }
        if carry {
            ret.insert_str(0, "1");
        }
        ret = String::from(ret.trim_start_matches("0"));
        if ret.len() == 0 {
            return String::from("0");
        }

        if negative_first {
            ret.insert_str(0, "-");
        }
        return ret;

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

        let v1 = read_from_end_and_pad(_f, max_len);
        let v2 = read_from_end_and_pad(_s, max_len);
        
        let mut chain = v1.zip(v2);
        let mut ret = String::from("");
        let mut carry = false;
        while let Some((x, y)) = chain.next() {
            let mut real_y = y;
            if carry {
                real_y = real_y + 1;
            }
            if real_y > x {
                carry = true;
                ret.insert_str(0, &((10 + x) - real_y).to_string());
            } else {
                ret.insert_str(0, &(x - real_y).to_string());
                carry = false;
            }
        }

        ret = String::from(ret.trim_start_matches("0"));

        if _f == _first_abs && negative_first ||
           _f == _second_abs && negative_second {
            ret.insert_str(0, "-");
        }
        return ret;
    }
}

pub fn subtract(_first: &str, _second: &str) -> String {
    // flip the second symbol sign and then do an add
    let negative_second = is_negative(_second);
    if negative_second {
        return add(_first, &_second[1.._second.len()]);
    } else {
        return add(_first, &format!("-{}", _second));
    }
}

pub fn pow_10(_num: &str, _pow: usize) -> String {
    let mut s = String::from(_num);
    for _ in 0.._pow {
        s.push_str("0");
    }
    return s;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pow_10() {
        assert_eq!(super::pow_10("2", 2), "200");
    }

    #[test]
    fn test_add() {
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
    fn test_max() {
        assert_eq!(super::max("2", "-3"), "2");
        assert_eq!(super::max("-2", "3"), "3");
        assert_eq!(super::max("2", "7"), "7");
        assert_eq!(super::max("22", "17051"), "17051");
        assert_eq!(super::max("-22", "-17051"), "-22");
    }
    
    #[test]
    fn test_multiply() {
        assert_eq!(super::multiply("0", "45"), "0");
        assert_eq!(super::multiply("0000", "45"), "0");
    }

    #[test]
    fn test_subtract() {
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