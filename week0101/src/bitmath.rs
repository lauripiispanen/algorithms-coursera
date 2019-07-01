pub fn add(_first: &[bool], _second: &[bool]) -> Vec<bool> {
    let _first_len = _first.len();
    let _second_len = _second.len();
    let len = std::cmp::max(_first_len, _second_len);

    let mut ret = Vec::with_capacity(len + 1);
    let mut carry = false;
    let mut first_non_false = 0;

    for i in 0..len {
        let _a = if i < _first_len { _first[_first_len - i - 1] } else { false };
        let _b = if i < _second_len { _second[_second_len - i - 1] } else { false };

        let v = (carry && (_a == _b)) || (!carry && (_a ^ _b));
        if v { first_non_false = i };
        carry = (_a && _b) || (carry && (_a || _b));
        ret.push(v);
    }
    if carry {
        ret.push(true);
    } else if len > 0 {
        ret.truncate(first_non_false + 1);
    }

    ret.reverse();
    return ret;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bitmath_add() {
        assert_eq!(add(&vec![true, false], &vec![true, true]), vec![true, false, true]);
        assert_eq!(add(&vec![true, true, true, false], &vec![true, true]), vec![true, false, false, false, true]);
        assert_eq!(add(&vec![false, false, true, false], &vec![true, true]), vec![true, false, true]);
        assert_eq!(add(&vec![true], &vec![]), vec![true]);
        assert_eq!(add(&vec![false, false, true], &vec![]), vec![true]);
        assert_eq!(add(&vec![], &vec![]), vec![]);
    }
/*    
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
*/
}