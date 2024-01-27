use std::fmt;
use std::ops::{Add, Sub, Neg, Mul};
use std::str::FromStr;
use super::{Bal3, TryFromIntError, ParseBalTernError, find_first_non_zero};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T10 {
    // contained in an i16
    value: [Bal3; 10]
}

impl fmt::Display for T10 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut strings: [String; 10] = Default::default();
        for item in self.value.iter().enumerate() {
            let (i, x) = item;
            strings[i] = x.to_string();
        }
        write!(f, "{}", strings.concat().trim_start_matches('0'))
    }
}

impl FromStr for T10 {
    type Err = ParseBalTernError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();
        if len == 0 {
            Err(ParseBalTernError::Empty)
        } else if len > 10 {
            match s.chars().next() {
                Some('T') => return Err(ParseBalTernError::NegOverflow),
                Some('1') => return Err(ParseBalTernError::PosOverflow),
                Some(_) => return Err(ParseBalTernError::InvalidChar),
                None => unreachable!()
            }
        } else {
            let zeros = 10 - len;
            let mut val = [Bal3::Zero; 10];
            for (i, c) in s.chars().enumerate() {
                val[i+zeros] = match c {
                    'T' => Bal3::NegativeOne,
                    '0' => Bal3::Zero,
                    '1' => Bal3::One,
                    _ => return Err(ParseBalTernError::InvalidChar),
                }
            }
            Ok(T10 {value: val})
        }
    }

}

impl T10 {
    pub const MAX: i16 = 29524;
    pub const MIN: i16 = -29524;
}

impl TryFrom<i16> for T10 {
    type Error = TryFromIntError;

    fn try_from(int: i16) -> Result<T10, TryFromIntError> {
        if int > T10::MAX {
            Err(TryFromIntError::PosOverflow)
        } else if int < T10::MIN {
            Err(TryFromIntError::NegOverflow)
        } else if int == 0 {
            Ok(T10 { value: [Bal3::Zero; 10] })
        } else {
            let mut value = [Bal3::Zero; 10];
            let mut remainder = int;
            while remainder.unsigned_abs() > 0 {
                let (pow, pos) = find_first_non_zero(remainder as isize);
                if pos {
                    remainder -= 3_u8.pow(pow as u32) as i16;
                    value[9 - pow as usize] = Bal3::One
                } else {
                    remainder += 3_u8.pow(pow as u32) as i16;
                    value[9 - pow as usize] = Bal3::NegativeOne
                }
            }
            Ok(T10 {value})
        }
    }
}

impl From<T10> for i16 {
    fn from(input: T10) -> Self {
        let mut acc: i16 = 0;
        for item in input.value.iter().enumerate() {
            let (i, x) = item;
            acc = match x {
                Bal3::One => acc + 3_i16.pow(9_u32-u32::try_from(i).expect("index should be < 10")),
                Bal3::NegativeOne => acc - 3_i16.pow(9_u32-u32::try_from(i).expect("index should be < 10")),
                Bal3::Zero => acc,
            };
        }
        acc
    }
}

impl From<T10> for String {
    fn from(input: T10) -> Self {
        input.to_string()
    }
}

impl Add for T10 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        T10::try_from(i16::from(self)+i16::from(rhs)).expect("Integer Overflow")
    }
}

impl Sub for T10 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        T10::try_from(i16::from(self)-i16::from(rhs)).expect("Integer Underflow")
    } 
}

impl Mul for T10 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        T10::try_from(i16::from(self)*i16::from(rhs)).expect("Out of Bounds")
    } 
}

impl Neg for T10 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        T10 {value: [Bal3::Zero; 10]} - self
    }
}

#[cfg(test)]
mod t10_tests {
    use super::*;

    #[test]
    fn from_i16() {
        assert_eq!(T10::try_from(6), Ok(T10 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T10::try_from(-48), Ok(T10 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::NegativeOne, Bal3::One, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T10::try_from(0), Ok(T10 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero]}));
        assert_eq!(T10::try_from(T10::MAX + 1), Err(TryFromIntError::PosOverflow));
        assert_eq!(T10::try_from(T10::MIN - 1), Err(TryFromIntError::NegOverflow));
    }

    #[test]
    fn t10_display() {
        let six = T10::try_from(6).unwrap();
        assert_eq!(format!("{:?}", six), "T10 { value: [Zero, Zero, Zero, Zero, Zero, Zero, Zero, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", six), "1T0");
        let neg_forty_eight = T10::try_from(-48).unwrap();
        assert_eq!(format!("{:?}", neg_forty_eight), "T10 { value: [Zero, Zero, Zero, Zero, Zero, NegativeOne, One, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", neg_forty_eight), "T11T0");
    }    

    #[test]
    fn t10_parse() {
        let neg_forty_eight: T10 = "T11T0".parse().unwrap();
        assert_eq!(T10::try_from(-48).unwrap(), neg_forty_eight);
        assert_eq!(T10::try_from(6).unwrap(), "1T0".parse::<T10>().unwrap());
        assert_eq!(ParseBalTernError::Empty, "".parse::<T10>().unwrap_err());
        assert_eq!(ParseBalTernError::PosOverflow, "10T01100000".parse::<T10>().unwrap_err());
        assert_eq!(ParseBalTernError::NegOverflow, "T0T01100000".parse::<T10>().unwrap_err());
        assert_eq!(ParseBalTernError::InvalidChar, "q01T0100000".parse::<T10>().unwrap_err());
        assert_eq!(ParseBalTernError::InvalidChar, "11T#1".parse::<T10>().unwrap_err());
    }

    #[test]
    fn addition() {
        assert_eq!(T10::try_from(6).unwrap()+T10::try_from(18).unwrap(),T10::try_from(24).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_overflow() {
        let _ = T10::try_from(T10::MAX).unwrap()+T10::try_from(1).unwrap();
    }

    #[test]
    fn subtraction() {
        assert_eq!(T10::try_from(6).unwrap()-T10::try_from(18).unwrap(),T10::try_from(-12).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_underflow() {
        let _ = T10::try_from(T10::MIN).unwrap()-T10::try_from(1).unwrap();
    }

    #[test]
    fn negation() {
        assert_eq!(-T10::try_from(6).unwrap(),T10::try_from(-6).unwrap())
    }

    #[test]
    fn multiplication() {
        assert_eq!(T10::try_from(6).unwrap()*T10::try_from(18).unwrap(),T10::try_from(108).unwrap())
    }

    #[test]
    #[should_panic]
    fn integer_oob() {
        let _ = T10::try_from(100).unwrap()*T10::try_from(300).unwrap();
    }


}
