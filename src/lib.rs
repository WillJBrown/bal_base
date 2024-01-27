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

#[derive(Debug, PartialEq)]
pub enum ParseBalTernError {
    PosOverflow,
    NegOverflow,
    InvalidChar,
    Empty,
}

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

/* fn calc_max_int_from_length(bits: u8) -> u64 {
    let mut acc: u64 = 0;
    for i in 0..bits {
        acc += 3_u64.pow(i as u32);
        println!("{}", acc);
    }
    acc
} */

mod t5;
mod t10;
mod t20;
mod t40;

#[cfg(test)]
mod tests {
    use super::*;
    use t5::T5;

    #[test]
    fn general() {
        //println!("Max num is {}", calc_max_int_from_length(40));

        // T01 * 1T -> -8 * 2 = -16 -> -27 + 9 + 3 -1 -> T11T
        let result = "T01".parse::<T5>().unwrap() * "1T".parse::<T5>().unwrap();
        println!("Result is {}", result);
    }

    #[test]
    fn bal3_display() {
        let display_one = format!("{}", Bal3::One);
        let display_zero = format!("{}", Bal3::Zero);
        let display_negone = format!("{}", Bal3::NegativeOne);
        assert_eq!(display_one, "1");
        assert_eq!(display_zero, "0");
        assert_eq!(display_negone, "T");
    }

    #[test]
    fn bal3_debug() {
        let debug_one = format!("{:?}", Bal3::One);
        let debug_zero = format!("{:?}", Bal3::Zero);
        let debug_negone = format!("{:?}", Bal3::NegativeOne);
        assert_eq!(debug_one, "One");
        assert_eq!(debug_zero, "Zero");
        assert_eq!(debug_negone, "NegativeOne");        
    }

    #[test]
    fn try_from_int_error_display() {
        let pos = TryFromIntError::PosOverflow;
        let neg = TryFromIntError::NegOverflow;
        assert_eq!(format!("{:?}", pos), "PosOverflow");
        assert_eq!(format!("{:?}", neg), "NegOverflow");
    }

}
