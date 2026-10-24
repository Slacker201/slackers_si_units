use fastnum::decimal::Context;
use si_unit_parser::Digit;

use crate::Backer;

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

        match fastnum::D64::from_str(&s, Context::default()) {
            Ok(v) => v.to_f64(),
            Err(e) => panic!("{e:?}"),
        }
    }
}
