use std::fmt;
use std::ops::{Add, Sub, Neg, Mul};
use std::str::FromStr;
use super::{Bal3, TryFromIntError, ParseBalTernError, find_first_non_zero};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T20 {
    // contained in an i32
    value: [Bal3; 20]
}

impl T20 {
    pub const MAX: i32 = 1743392200;
    pub const MIN: i32 = -1743392200;
}

impl fmt::Display for T20 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut strings: [String; 20] = Default::default();
        for item in self.value.iter().enumerate() {
            let (i, x) = item;
            strings[i] = x.to_string();
        }
        write!(f, "{}", strings.concat().trim_start_matches('0'))
    }
}

impl FromStr for T20 {
    type Err = ParseBalTernError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();
        if len == 0 {
            Err(ParseBalTernError::Empty)
        } else if len > 20 {
            match s.chars().next() {
                Some('T') => return Err(ParseBalTernError::NegOverflow),
                Some('1') => return Err(ParseBalTernError::PosOverflow),
                Some(_) => return Err(ParseBalTernError::InvalidChar),
                None => unreachable!()
            }
        } else {
            let zeros = 20 - len;
            let mut val = [Bal3::Zero; 20];
            for (i, c) in s.chars().enumerate() {
                val[i+zeros] = match c {
                    'T' => Bal3::NegativeOne,
                    '0' => Bal3::Zero,
                    '1' => Bal3::One,
                    _ => return Err(ParseBalTernError::InvalidChar),
                }
            }
            Ok(T20 {value: val})
        }
    }

}

impl TryFrom<i32> for T20 {
    type Error = TryFromIntError;

    fn try_from(int: i32) -> Result<T20, TryFromIntError> {
        if int > T20::MAX {
            Err(TryFromIntError::PosOverflow)
        } else if int < T20::MIN {
            Err(TryFromIntError::NegOverflow)
        } else if int == 0 {
            Ok(T20 { value: [Bal3::Zero; 20] })
        } else {
            let mut value = [Bal3::Zero; 20];
            let mut remainder = int;
            while remainder.unsigned_abs() > 0 {
                let (pow, pos) = find_first_non_zero(remainder as isize);
                if pos {
                    remainder -= 3_u32.pow(pow as u32) as i32;
                    value[19 - pow as usize] = Bal3::One
                } else {
                    remainder += 3_u32.pow(pow as u32) as i32;
                    value[19 - pow as usize] = Bal3::NegativeOne
                }
            }
            Ok(T20 {value})
        }
    }
}

impl From<T20> for i32 {
    fn from(input: T20) -> Self {
        let mut acc: i32 = 0;
        for item in input.value.iter().enumerate() {
            let (i, x) = item;
            acc = match x {
                Bal3::One => acc + 3_i32.pow(19_u32-u32::try_from(i).expect("index should be < 20")),
                Bal3::NegativeOne => acc - 3_i32.pow(19_u32-u32::try_from(i).expect("index should be < 20")),
                Bal3::Zero => acc,
            };
        }
        acc
    }
}

impl From<T20> for String {
    fn from(input: T20) -> Self {
        input.to_string()
    }
}

impl Add for T20 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        T20::try_from(i32::from(self)+i32::from(rhs)).expect("Integer Overflow")
    }
}

impl Sub for T20 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        T20::try_from(i32::from(self)-i32::from(rhs)).expect("Integer Underflow")
    } 
}

impl Neg for T20 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        T20 {value: [Bal3::Zero; 20]} - self
    }
}

impl Mul for T20 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        T20::try_from(i32::from(self)*i32::from(rhs)).expect("Out of Bounds")
    } 
}

#[cfg(test)]
mod t20_tests {
    use super::*;

    #[test]
    fn from_i32() {
        assert_eq!(T20::try_from(6), Ok(T20 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T20::try_from(-48), Ok(T20 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::NegativeOne, Bal3::One, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T20::try_from(0), Ok(T20 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero]}));
        assert_eq!(T20::try_from(T20::MAX + 1), Err(TryFromIntError::PosOverflow));
        assert_eq!(T20::try_from(T20::MIN -1), Err(TryFromIntError::NegOverflow));
    }

    #[test]
    fn t20_display() {
        let six = T20::try_from(6).unwrap();
        assert_eq!(format!("{:?}", six), "T20 { value: [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", six), "1T0");
        assert_eq!(String::from(six), "1T0");
        let neg_forty_eight = T20::try_from(-48).unwrap();
        assert_eq!(format!("{:?}", neg_forty_eight), "T20 { value: [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, NegativeOne, One, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", neg_forty_eight), "T11T0");
        assert_eq!(String::from(neg_forty_eight), "T11T0");
    }    

    #[test]
    fn t20_parse() {
        let neg_forty_eight: T20 = "T11T0".parse().unwrap();
        assert_eq!(T20::try_from(-48).unwrap(), neg_forty_eight);
        assert_eq!(T20::try_from(6).unwrap(), "1T0".parse::<T20>().unwrap());
        assert_eq!(ParseBalTernError::Empty, "".parse::<T20>().unwrap_err());
        assert_eq!(ParseBalTernError::PosOverflow, "10T011000000000000000".parse::<T20>().unwrap_err());
        assert_eq!(ParseBalTernError::NegOverflow, "T0T011000000000000000".parse::<T20>().unwrap_err());
        assert_eq!(ParseBalTernError::InvalidChar, "q01T01000000000000000".parse::<T20>().unwrap_err());
        assert_eq!(ParseBalTernError::InvalidChar, "11T#1".parse::<T20>().unwrap_err());
    }

    #[test]
    fn addition() {
        assert_eq!(T20::try_from(6).unwrap()+T20::try_from(18).unwrap(),T20::try_from(24).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_overflow() {
        let _ = T20::try_from(T20::MAX).unwrap()+T20::try_from(1).unwrap();
    }

    #[test]
    fn subtraction() {
        assert_eq!(T20::try_from(6).unwrap()-T20::try_from(18).unwrap(),T20::try_from(-12).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_underflow() {
        let _ = T20::try_from(T20::MIN).unwrap()-T20::try_from(1).unwrap();
    }

    #[test]
    fn negation() {
        assert_eq!(-T20::try_from(6).unwrap(),T20::try_from(-6).unwrap())
    }

    #[test]
    fn multiplication() {
        assert_eq!(T20::try_from(6).unwrap()*T20::try_from(18).unwrap(),T20::try_from(108).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_oob() {
        let _ = T20::try_from(18).unwrap()*T20::try_from(100000000).unwrap();
    }


}
