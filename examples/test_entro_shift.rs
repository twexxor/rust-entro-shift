extern crate entro_shift;

use entro_shift::entro_shift::entro_shift;

fn main() {
    let mut entropy = entro_shift(1111111111);
    let mut i: usize = 0;

    while i != 10 {
        entropy = entro_shift(entropy);
        println!("{}", entropy);
        i += 1;
    }
}
