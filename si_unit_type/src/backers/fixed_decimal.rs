use fixed::{FixedI128, types::extra::{U8, U16, U32, U64, U96}};

use crate::Backer;


macro_rules! implement_fixed {
    ($t: ident) => {
        impl Backer for FixedI128<$t> {
            fn from_fixed_decimal(integer: &[si_unit_parser::Digit], decimal: &[si_unit_parser::Digit]) -> Self {
                let mut s = String::new();
                for digit in integer {
                    s.push(digit.to_char());
                }
                s.push('.');
                for digit in decimal {
                    s.push(digit.to_char());
                }

                Self::from_str(&s).unwrap()
            }
        }
    };
}

implement_fixed!(U8);
implement_fixed!(U16);
implement_fixed!(U32);
implement_fixed!(U64);
implement_fixed!(U96);