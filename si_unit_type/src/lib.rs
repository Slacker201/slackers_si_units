use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use si_unit_parser::Digit;



pub trait Backer: Add + AddAssign + Sub + SubAssign + Div + DivAssign + Mul + MulAssign + Clone {
    fn from_fixed_decimal(integer: &[Digit], decimal: &[Digit]) -> Self;
}

pub struct Gram<B: Backer> {
    backer: B,
}

impl<B: Backer + Debug> Debug for Gram<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gram").field("backer", &self.backer).finish()
    }
}

impl<B: Backer> Gram<B> {
    pub const fn new(backer: B) -> Self {
        Self { backer }
    }
}

impl Backer for f64 {
    fn from_fixed_decimal(integer: &[Digit], decimal: &[Digit]) -> Self {
        let mut s = String::new();
        for int in integer {
            s.push(int.to_char());
        }
        s.push('.');
        for int in decimal{
            s.push(int.to_char());
        }

        s.parse().unwrap()
    }
}