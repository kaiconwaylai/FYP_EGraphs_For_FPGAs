use std::ops::Add;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Cost {
    pub dsp : i32,
    pub lut : i32
}

impl Add for Cost {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            dsp: self.dsp + other.dsp,
            lut: self.lut + other.lut,
        }
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering  {
        if self.dsp < other.dsp {
            return Ordering::Less;
        } else if self.dsp == other.dsp && self.lut < other.lut {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
}

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(dsps: {}, luts: {})", self.dsp, self.lut)
    }
}