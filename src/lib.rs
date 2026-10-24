pub use si_unit_macro::unit;
pub use si_unit_parser::Digit;
pub use si_unit_type::Backer;
pub use si_unit_type::{Gram, Liter, Meter, Second};

#[macro_export]
macro_rules! register_backer {
    ($t: ty) => {
        #[allow(non_camel_case_types)]
        type __BACKER_TYPE__ = $t;
    };
}
