use std::fmt;
use std::ops::{Add, Sub, Neg, Mul};
use std::str::FromStr;
use super::{Bal3, TryFromIntError, ParseBalTernError, find_first_non_zero};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T40 {
    // contained in an i64
    value: [Bal3; 40]
}

impl fmt::Display for T40 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut strings: Vec<String> = vec![String::from(""); 40];
        for item in self.value.iter().enumerate() {
            let (i, x) = item;
            strings[i] = x.to_string();
        }
        write!(f, "{}", strings.concat().trim_start_matches('0'))
    }
}

impl FromStr for T40 {
    type Err = ParseBalTernError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();
        if len == 0 {
            Err(ParseBalTernError::Empty)
        } else if len > 40 {
            match s.chars().next() {
                Some('T') => return Err(ParseBalTernError::NegOverflow),
                Some('1') => return Err(ParseBalTernError::PosOverflow),
                Some(_) => return Err(ParseBalTernError::InvalidChar),
                None => unreachable!()
            }
        } else {
            let zeros = 40 - len;
            let mut val = [Bal3::Zero; 40];
            for (i, c) in s.chars().enumerate() {
                val[i+zeros] = match c {
                    'T' => Bal3::NegativeOne,
                    '0' => Bal3::Zero,
                    '1' => Bal3::One,
                    _ => return Err(ParseBalTernError::InvalidChar),
                }
            }
            Ok(T40 {value: val})
        }
    }

}

impl T40 {
    pub const MAX: i64 = 6078832729528464400;
    pub const MIN: i64 = -6078832729528464400;
}

impl TryFrom<i64> for T40 {
    type Error = TryFromIntError;

    fn try_from(int: i64) -> Result<T40, TryFromIntError> {
        if int > T40::MAX {
            Err(TryFromIntError::PosOverflow)
        } else if int < T40::MIN {
            Err(TryFromIntError::NegOverflow)
        } else if int == 0 {
            Ok(T40 { value: [Bal3::Zero; 40] })
        } else {
            let mut value = [Bal3::Zero; 40];
            let mut remainder = int;
            while remainder.unsigned_abs() > 0 {
                let (pow, pos) = find_first_non_zero(remainder as isize);
                if pos {
                    remainder -= 3_u64.pow(pow as u32) as i64;
                    value[39 - pow as usize] = Bal3::One
                } else {
                    remainder += 3_u64.pow(pow as u32) as i64;
                    value[39 - pow as usize] = Bal3::NegativeOne
                }
            }
            Ok(T40 {value})
        }
    }
}


impl From<T40> for i64 {
    fn from(input: T40) -> Self {
        let mut acc: i64 = 0;
        for item in input.value.iter().enumerate() {
            let (i, x) = item;
            acc = match x {
                Bal3::One => acc + 3_i64.pow(39_u32-u32::try_from(i).expect("index should be < 40")),
                Bal3::NegativeOne => acc - 3_i64.pow(39_u32-u32::try_from(i).expect("index should be < 40")),
                Bal3::Zero => acc,
            };
        }
        acc
    }
}

impl From<T40> for String {
    fn from(input: T40) -> Self {
        input.to_string()
    }
}

impl Add for T40 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        T40::try_from(i64::from(self)+i64::from(rhs)).expect("Integer Overflow")
    }
}

impl Sub for T40 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        T40::try_from(i64::from(self)-i64::from(rhs)).expect("Integer Underflow")
    } 
}

impl Neg for T40 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        T40 {value: [Bal3::Zero; 40]} - self
    }
}

impl Mul for T40 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        T40::try_from(i64::from(self)*i64::from(rhs)).expect("Out of Bounds")
    } 
}

#[cfg(test)]
mod t40_tests {
    use super::*;

    #[test]
    fn from_i64() {
        assert_eq!(T40::try_from(6), Ok(T40 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T40::try_from(-48), Ok(T40 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::NegativeOne, Bal3::One, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T40::try_from(0), Ok(T40 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero]}));
        assert_eq!(T40::try_from(T40::MAX + 1), Err(TryFromIntError::PosOverflow));
        assert_eq!(T40::try_from(T40::MIN - 1), Err(TryFromIntError::NegOverflow));
    }

    #[test]
    fn t40_display() {
        let six = T40::try_from(6).unwrap();
        assert_eq!(format!("{:?}", six), "T40 { value: [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", six), "1T0");
        assert_eq!(String::from(six), "1T0");
        let neg_forty_eight = T40::try_from(-48).unwrap();
        assert_eq!(format!("{:?}", neg_forty_eight), "T40 { value: [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, NegativeOne, One, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", neg_forty_eight), "T11T0");
        assert_eq!(String::from(neg_forty_eight), "T11T0");
    }    

    #[test]
    fn t40_parse() {
        let neg_forty_eight: T40 = "T11T0".parse().unwrap();
        assert_eq!(T40::try_from(-48).unwrap(), neg_forty_eight);
        assert_eq!(T40::try_from(6).unwrap(), "1T0".parse::<T40>().unwrap());
        assert_eq!(ParseBalTernError::Empty, "".parse::<T40>().unwrap_err());
        assert_eq!(ParseBalTernError::PosOverflow, "10T01100000000000000000000000000000000000".parse::<T40>().unwrap_err());
        assert_eq!(ParseBalTernError::NegOverflow, "T0T01100000000000000000000000000000000000".parse::<T40>().unwrap_err());
        assert_eq!(ParseBalTernError::InvalidChar, "q01T0100000000000000000000000000000000000".parse::<T40>().unwrap_err());
        assert_eq!(ParseBalTernError::InvalidChar, "11T#1".parse::<T40>().unwrap_err());
    }

    #[test]
    fn addition() {
        assert_eq!(T40::try_from(6).unwrap()+T40::try_from(18).unwrap(),T40::try_from(24).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_overflow() {
        let _ = T40::try_from(T40::MAX).unwrap()+T40::try_from(1).unwrap();
    }

    #[test]
    fn subtraction() {
        assert_eq!(T40::try_from(6).unwrap()-T40::try_from(18).unwrap(),T40::try_from(-12).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_underflow() {
        let _ = T40::try_from(T40::MIN).unwrap()-T40::try_from(1).unwrap();
    }

    #[test]
    fn negation() {
        assert_eq!(-T40::try_from(6).unwrap(),T40::try_from(-6).unwrap())
    }

    #[test]
    fn multiplication() {
        assert_eq!(T40::try_from(6).unwrap()*T40::try_from(18).unwrap(),T40::try_from(108).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_oob() {
        let _ = T40::try_from(61).unwrap()*T40::try_from(1000000000000000000).unwrap();
    }


}
