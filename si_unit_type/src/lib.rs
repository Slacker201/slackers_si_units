use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use si_unit_parser::Digit;

pub trait Backer:
    Add + AddAssign + Sub + SubAssign + Div + DivAssign + Mul + MulAssign + Clone
{
    fn from_fixed_decimal(integer: &[Digit], decimal: &[Digit]) -> Self;
}

macro_rules! create_unit {
    ($name: ident, $name_but_str: literal) => {
        pub struct $name<B: Backer> {
            backer: B,
        }

        impl<B: Backer + Debug> Debug for $name<B> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct($name_but_str)
                    .field("backer", &self.backer)
                    .finish()
            }
        }

        impl<B: Backer> $name<B> {
            pub const fn new(backer: B) -> Self {
                Self { backer }
            }
        }
    };
}

create_unit!(Gram, "Gram");
create_unit!(Meter, "Meter");
create_unit!(Second, "Second");
create_unit!(Liter, "Liter");

impl Backer for f64 {
    fn from_fixed_decimal(integer: &[Digit], decimal: &[Digit]) -> Self {
        let mut s = String::new();
        for int in integer {
            s.push(int.to_char());
        }
        s.push('.');
        for int in decimal {
            s.push(int.to_char());
        }

        s.parse().unwrap()
    }
}
