# Entro Shift
## Description
Entro Shift is a 32-bit pseudo-random number generator algorithm.

## Code Example
The following code demonstrates an example implementation included in this package with `1111111111` as the seed.

``` rust
extern crate entro_shift;

use entro_shift::entro_shift::entro_shift;

fn main() {
    let mut entropy = entro_shift(1111111111);
    let mut i: usize = 0;

    while i != 10 {
        entropy = entro_shift(entropy);
        i += 1;
    }

    println!("{}", entropy);
}
```

To test the Cargo package, execute the following command.

``` console
cargo test
```

## Purchase
[EntroCraft](https://entrocraft.com/) maintains this open-source package with the permissive MIT license.

It's provided as a convenient code evaluation tool for the [premium Entro Shift library written in C](https://entrocraft.com/dungeon/randomization-algorithms/entro-shift/).

Converting code in this package from Rust to another programming language is discouraged and may be subject to either [purchasing a license](https://entrocraft.com/dungeon/randomization-algorithms/entro-shift/#license) for the premium library in C or attributing other OSI licenses.

Developers who don't use the C programming language can still [purchase credits](https://entrocraft.com/pricing/), learn C and support package maintenance.
