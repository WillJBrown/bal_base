use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bal3 {
    One,
    Zero,
    NegativeOne,
}

impl fmt::Display for Bal3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bal3::One => write!(f, "{}", String::from('1')),
            Bal3::Zero => write!(f, "{}", String::from('0')),
            Bal3::NegativeOne => write!(f, "{}", String::from('T')),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum TryFromIntError {
    PosOverflow,
    NegOverflow,
}

enum ParseBalTernError {
    PosOverflow,
    NegOverflow,
    InvalidChar,
    Empty,
}

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

impl T5 {
    pub const MAX: i8 = 121;
    pub const MIN: i8 = -121;

    pub fn to_i8(&self) -> i8 {
        let mut acc: i8 = 0;
        for item in self.value.iter().enumerate() {
            let (i, x) = item;
            acc = match x {
                Bal3::One => acc + 3_i8.pow(4_u32-u32::try_from(i).expect("index should be < 5")),
                Bal3::NegativeOne => acc - 3_i8.pow(4_u32-u32::try_from(i).expect("index should be < 5")),
                Bal3::Zero => acc,
            };
        }
        acc
    }

    pub fn from_i8(int: i8) -> Result<T5, TryFromIntError> {
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

// Not yet implemented correctly
fn find_first_non_zero(int: isize) -> (u8, bool) {
    let pos = int >= 0;
    let modulus: usize = int.unsigned_abs();
    let mut max_num: usize = 0;
    let mut index: u8 = 0;
    for pow in 0..40_u8 {
        max_num += 3_usize.pow(pow as u32);
        if max_num >= modulus {
            index = pow;
            break;
        }
    }
    (index, pos)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T10{
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T20{
    // contained in an i32
    value: [Bal3; 20],
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T40{
    // contained in an i64
    value: [Bal3; 40],
}

impl fmt::Display for T40 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const EMPTY_STRING: String = String::new();
        let mut strings: [String; 40] = [EMPTY_STRING; 40];
        for item in self.value.iter().enumerate() {
            let (i, x) = item;
            strings[i] = x.to_string();
        }
        write!(f, "{}", strings.concat().trim_start_matches('0'))
    }
}

pub fn add(left: T5, right: T5) -> Result<T5, TryFromIntError> {
    T5::from_i8(left.to_i8()+right.to_i8())
}

pub fn main() {
    println!("{:?}", Bal3::One);
    println!("{}", Bal3::One);
    println!("{}", Bal3::NegativeOne);
    println!("1T011 plus TT011 is: {}", add(T5 {value: [Bal3::One, Bal3::NegativeOne, Bal3::Zero, Bal3::One, Bal3::One]}, T5 {value: [Bal3::NegativeOne, Bal3::NegativeOne, Bal3::Zero, Bal3::One, Bal3::One]}).unwrap());
    println!("{}", T5::from_i8(6).unwrap());
    println!("{}", T5::from_i8(6).unwrap().to_i8());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_main() {
        main();
    }

    #[test]
    fn display_bal3_test() {
        let debug_one = format!("{:?}", Bal3::One);
        let debug_zero = format!("{:?}", Bal3::Zero);
        let debug_negone = format!("{:?}", Bal3::NegativeOne);
        assert_eq!(debug_one, "One");
        assert_eq!(debug_zero, "Zero");
        assert_eq!(debug_negone, "NegativeOne");
        let display_one = format!("{}", Bal3::One);
        let display_zero = format!("{}", Bal3::Zero);
        let display_negone = format!("{}", Bal3::NegativeOne);
        assert_eq!(display_one, "1");
        assert_eq!(display_zero, "0");
        assert_eq!(display_negone, "T");
    }

    #[test]
    fn display_t5_test() {
        let six = T5 {value: [Bal3::Zero, Bal3::Zero, Bal3::One, Bal3::NegativeOne, Bal3::Zero]};
        assert_eq!(format!("{:?}", six), "T5 { value: [Zero, Zero, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", six), "1T0");
        let neg_forty_eight = T5 {value: [Bal3::NegativeOne, Bal3::One, Bal3::One, Bal3::NegativeOne, Bal3::Zero]};
        assert_eq!(format!("{:?}", neg_forty_eight), "T5 { value: [NegativeOne, One, One, NegativeOne, Zero] }");
        assert_eq!(format!("{}", neg_forty_eight), "T11T0");
    }

    #[test]
    fn display_try_from_int_error_test() {
        let pos = TryFromIntError::PosOverflow;
        let neg = TryFromIntError::NegOverflow;
        assert_eq!(format!("{:?}", pos), "PosOverflow");
        assert_eq!(format!("{:?}", neg), "NegOverflow");
    }

    #[test]
    fn from_i8_test() {
        assert_eq!(T5::from_i8(6), Ok(T5 {value: [Bal3::Zero, Bal3::Zero, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T5::from_i8(-48), Ok(T5 {value: [Bal3::NegativeOne, Bal3::One, Bal3::One, Bal3::NegativeOne, Bal3::Zero]}));
        assert_eq!(T5::from_i8(0), Ok(T5 {value: [Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero, Bal3::Zero]}));
        assert_eq!(T5::from_i8(122), Err(TryFromIntError::PosOverflow));
        assert_eq!(T5::from_i8(-122), Err(TryFromIntError::NegOverflow));
    }


}
