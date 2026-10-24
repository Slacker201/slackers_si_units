use fixed::{FixedI128, types::extra::U64};
use slackers_si_units::{register_backer, unit};

register_backer!(FixedI128<U64>);

fn main() {
    let q = FixedI128::<U64>::from_be_bytes([254; 16]);

    let m = unit!(9223372036854775807.9999999999999999999457898913757247782996273599565029144287109375l);

    println!("{:?}", m);
}
