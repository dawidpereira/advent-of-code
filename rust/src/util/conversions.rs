use std::fmt::Debug;

pub trait FromChar: Sized {
    type Err: Debug;

    fn from_char(c: char) -> Result<Self, Self::Err>;
}

impl FromChar for u32 {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c.to_digit(10) {
            Some(digit) => Ok(digit),
            None => Err(format!("Failed to parse '{}' as 32", c)),
        }
    }
}

impl FromChar for char {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        Ok(c)
    }
}

impl FromChar for usize {
    type Err = std::num::ParseIntError;
    fn from_char(c: char) -> Result<Self, Self::Err> {
        c.to_string().parse()
    }
}

impl FromChar for i32 {
    type Err = std::num::ParseIntError;
    fn from_char(c: char) -> Result<Self, Self::Err> {
        c.to_string().parse()
    }
}
