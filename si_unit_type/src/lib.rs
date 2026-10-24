use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use si_unit_parser::Digit;

pub trait Backer:
    Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + Div<Output = Self> + DivAssign + Mul<Output = Self> + MulAssign + Clone
{
    fn from_fixed_decimal(integer: &[Digit], decimal: &[Digit]) -> Self;
}

mod backers;

macro_rules! create_unit {
    ($name: ident, $name_but_str: literal) => {
        #[derive(Clone, PartialEq, PartialOrd)]
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

        impl<B: Backer> Add<$name<B>> for $name<B> {
            type Output = Self;

            fn add(self, rhs: $name<B>) -> Self::Output {
                Self {
                    backer: self.backer + rhs.backer
                }
            }
        }

        impl<B: Backer> Sub<$name<B>> for $name<B> {
            type Output = Self;

            fn sub(self, rhs: $name<B>) -> Self::Output {
                Self {
                    backer: self.backer - rhs.backer
                }
            }
        }

        impl<B: Backer> Mul<Unit<B>> for $name<B> {
            type Output = Self;

            fn mul(self, rhs: Unit<B>) -> Self::Output {
                Self {
                    backer: self.backer * rhs.backer
                }
            }
        }

        impl<B: Backer> Div<Unit<B>> for $name<B> {
            type Output = Self;

            fn div(self, rhs: Unit<B>) -> Self::Output {
                Self {
                    backer: self.backer / rhs.backer
                }
            }
        }

        impl<B: Backer> AddAssign<$name<B>> for $name<B> {
            fn add_assign(&mut self, rhs: $name<B>) {
                self.backer += rhs.backer;
            }
        }
        impl<B: Backer> SubAssign<$name<B>> for $name<B> {
            fn sub_assign(&mut self, rhs: $name<B>) {
                self.backer -= rhs.backer;
            }
        }
        impl<B: Backer> MulAssign<Unit<B>> for $name<B> {
            fn mul_assign(&mut self, rhs: Unit<B>) {
                self.backer *= rhs.backer;
            }
        }
        impl<B: Backer> DivAssign<Unit<B>> for $name<B> {
            fn div_assign(&mut self, rhs: Unit<B>) {
                self.backer /= rhs.backer;
            }
        }

        impl<B: Backer + Copy + Clone> Copy for $name<B> {}
        impl<B: Backer + PartialEq + Eq> Eq for $name<B> {}
        impl<B: Backer + PartialOrd + Ord> Ord for $name<B> {
            fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                self.backer.cmp(&rhs.backer)
            }
        }
    };
}

create_unit!(Gram, "Gram");
create_unit!(Meter, "Meter");
create_unit!(Second, "Second");
create_unit!(Liter, "Liter");
create_unit!(Unit, "Unit");