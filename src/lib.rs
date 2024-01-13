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
#[derive(Debug)]
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
        write!(f, "{}", strings.concat())
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
            while remainder.unsigned_abs() > 1 {
                let (pow, pos) = find_first_non_zero(remainder as isize);
                if pos {
                        value[4 - pow as usize] = Bal3::One
                } else {
                    value[4 - pow as usize] = Bal3::NegativeOne
                }
                remainder = (3_u8.pow(pow as u32) as i8) - remainder;
            }
            match remainder {
                1 => value[4] = Bal3::One,
                0 => value[4] = Bal3::Zero,
                -1 => value[4] = Bal3::NegativeOne,
                _ => panic!("This should never be called because the while loop ensures remainder.abs() <= 1 by this point"),
            }
            Ok(T5 {value})
        }
    }
}

// Not yet implemented correctly
fn find_first_non_zero(int: isize) -> (u8, bool) {
    if int == 0 { return (0, true)}
    let mut min_dist = 2 * T5::MAX as usize;
    let mut pos: bool = true;
    let mut index: u8 = 0;
    for i in 0..40 {
        let pow = 3_isize.pow(i).unsigned_abs() as isize;
        if ((int - pow) as usize) < min_dist {
            min_dist = (int - 3_isize.pow(i)).unsigned_abs();
            pos = true;
            index = i as u8;
        }
        if ((int + pow) as usize) < min_dist {
            min_dist = (int + 3_isize.pow(i)).unsigned_abs();
            pos = false;
            index = i as u8;
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
        write!(f, "{}", strings.concat())
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
        write!(f, "{}", strings.concat())
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
        write!(f, "{}", strings.concat())
    }
}

pub fn add(left: Bal3, right: Bal3) -> Bal3 {
    match left {
        Bal3::One => match right {
            Bal3::NegativeOne => Bal3::Zero,
            _ => Bal3::One,
        },
        Bal3::Zero => match right {
            Bal3::One => Bal3::One,
            Bal3::Zero => Bal3::Zero,
            Bal3::NegativeOne => Bal3::NegativeOne,
        },
        Bal3::NegativeOne => match right {
            Bal3::One => Bal3::Zero,
            _ => Bal3::NegativeOne,
        }
    }
}

pub fn main() {
    println!("{:?}", Bal3::One);
    println!("{}", Bal3::One);
    println!("{}", Bal3::NegativeOne);
    println!("One plus Zero is: {}", add(Bal3::One, Bal3::Zero));
    println!("{:?}", find_first_non_zero(6));
    //println!("{:?}", T5::from_i8(6).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_main() {
        main();
    }

    #[test]
    fn display_test() {
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
}
