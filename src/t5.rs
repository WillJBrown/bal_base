use std::fmt;
use std::ops::{Add, Sub, Neg, Mul};
use std::str::FromStr;
use super::{Bal3, TryFromIntError, ParseBalTernError, find_first_non_zero};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T5 {
    // contained in an i8
    value: [Bal3; 5]
}

impl fmt::Display for T5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut strings: [String; 5] = Default::default();
        for item in self.value.iter().enumerate() {
            let (i, x) = item;
            strings[i] = x.to_string();
        }
        write!(f, "{}", strings.concat().trim_start_matches('0'))
    }
}

impl FromStr for T5 {
    type Err = ParseBalTernError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();
        if len == 0 {
            Err(ParseBalTernError::Empty)
        } else if len > 5 {
            match s.chars().next() {
                Some('T') => return Err(ParseBalTernError::NegOverflow),
                Some('1') => return Err(ParseBalTernError::PosOverflow),
                Some(_) => return Err(ParseBalTernError::InvalidChar),
                None => unreachable!()
            }
        } else {
            let zeros = 5 - len;
            let mut val = [Bal3::Zero; 5];
            for (i, c) in s.chars().enumerate() {
                val[i+zeros] = match c {
                    'T' => Bal3::NegativeOne,
                    '0' => Bal3::Zero,
                    '1' => Bal3::One,
                    _ => return Err(ParseBalTernError::InvalidChar),
                }
            }
            Ok(T5 {value: val})
        }
    }

}

impl T5 {
    pub const MAX: i8 = 121;
    pub const MIN: i8 = -121;
}

impl TryFrom<i8> for T5 {
    type Error = TryFromIntError;

    fn try_from(int: i8) -> Result<T5, TryFromIntError> {
        if int > T5::MAX {
            Err(TryFromIntError::PosOverflow)
        } else if int < T5::MIN {
            Err(TryFromIntError::NegOverflow)
        } else if int == 0 {
            Ok(T5 { value: [Bal3::Zero; 5] })
        } else {
            let mut value = [Bal3::Zero; 5];
            let mut remainder = int;
            while remainder.unsigned_abs() > 0 {
                let (pow, pos) = find_first_non_zero(remainder as isize);
                if pos {
                    remainder -= 3_u8.pow(pow as u32) as i8;
                    value[4 - pow as usize] = Bal3::One
                } else {
                    remainder += 3_u8.pow(pow as u32) as i8;
                    value[4 - pow as usize] = Bal3::NegativeOne
                }
            }
            Ok(T5 {value})
        }
    }
}

impl From<T5> for i8 {
    fn from(input: T5) -> Self {
        let mut acc: i8 = 0;
        for item in input.value.iter().enumerate() {
            let (i, x) = item;
            acc = match x {
                Bal3::One => acc + 3_i8.pow(4_u32-u32::try_from(i).expect("index should be < 5")),
                Bal3::NegativeOne => acc - 3_i8.pow(4_u32-u32::try_from(i).expect("index should be < 5")),
                Bal3::Zero => acc,
            };
        }
        acc
    }
}

impl From<T5> for String {
    fn from(input: T5) -> Self {
        input.to_string()
    }
}

impl Add for T5 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        T5::try_from(i8::from(self)+i8::from(rhs)).expect("Integer Overflow")
    }
}

impl Sub for T5 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        T5::try_from(i8::from(self)-i8::from(rhs)).expect("Integer Underflow")
    } 
}

impl Mul for T5 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        T5::try_from(i8::from(self)*i8::from(rhs)).expect("Out of Bounds")
    } 
}

impl Neg for T5 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        T5 {value: [Bal3::Zero; 5]} - self
    }
}

#[cfg(test)]
mod t5_tests {
    use super::*;

    #[test]
    fn from_i8() {
        assert_eq!(T5::try_from(6), Ok(T5 {value: [Bal3::Zero, Bal3::Zero, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T5::try_from(-48), Ok(T5 {value: [Bal3::NegativeOne, Bal3::One, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T5::try_from(0), Ok(T5 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero]}));
        assert_eq!(T5::try_from(T5::MAX + 1), Err(TryFromIntError::PosOverflow));
        assert_eq!(T5::try_from(T5::MIN - 1), Err(TryFromIntError::NegOverflow));
    }

    #[test]
    fn t5_display() {
        let six = T5::try_from(6).unwrap();
        assert_eq!(format!("{:?}", six), "T5 { value: [Zero, Zero, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", six), "1T0");
        let neg_forty_eight = T5::try_from(-48).unwrap();
        assert_eq!(format!("{:?}", neg_forty_eight), "T5 { value: [NegativeOne, One, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", neg_forty_eight), "T11T0");
    }    

    #[test]
    fn t5_parse() {
        let neg_forty_eight: T5 = "T11T0".parse().unwrap();
        assert_eq!(T5::try_from(-48).unwrap(), neg_forty_eight);
        assert_eq!(T5::try_from(6).unwrap(), "1T0".parse::<T5>().unwrap());
        assert_eq!(ParseBalTernError::Empty, "".parse::<T5>().unwrap_err());
        assert_eq!(ParseBalTernError::PosOverflow, "10T011".parse::<T5>().unwrap_err());
        assert_eq!(ParseBalTernError::NegOverflow, "T0T011".parse::<T5>().unwrap_err());
        assert_eq!(ParseBalTernError::InvalidChar, "q01T01".parse::<T5>().unwrap_err());
        assert_eq!(ParseBalTernError::InvalidChar, "11T#1".parse::<T5>().unwrap_err());
    }

    #[test]
    fn addition() {
        assert_eq!(T5::try_from(6).unwrap()+T5::try_from(18).unwrap(),T5::try_from(24).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_overflow() {
        let _ = T5::try_from(T5::MAX).unwrap()+T5::try_from(1).unwrap();
    }

    #[test]
    fn subtraction() {
        assert_eq!(T5::try_from(6).unwrap()-T5::try_from(18).unwrap(),T5::try_from(-12).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_underflow() {
        let _ = T5::try_from(T5::MIN).unwrap()-T5::try_from(1).unwrap();
    }

    #[test]
    fn negation() {
        assert_eq!(-T5::try_from(6).unwrap(),T5::try_from(-6).unwrap())
    }

    #[test]
    fn multiplication() {
        assert_eq!(T5::try_from(6).unwrap()*T5::try_from(18).unwrap(),T5::try_from(108).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_oob() {
        let _ = T5::try_from(10).unwrap()*T5::try_from(13).unwrap();
    }


}
