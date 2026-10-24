use slackers_si_units::{register_backer, unit};

register_backer!(f64);

fn main() {
    let m = unit!(10ml);

    println!("{:?}", m);
}
