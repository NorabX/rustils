extern crate rustils;
extern crate rand;
use rustils::ArrayUtils;
use rand::{thread_rng, Rng, SeedableRng, StdRng};

fn main() {
    let x = &mut [1,2,3,4,5,6,7];
    let y = &mut [1,2,3,4,5,6,7];
    let mut rng = thread_rng();

    println!("{:?}", x);
    x.shuffle();
    println!("{:?}", x);

    rng.shuffle(y);
    println!("{:?}", y);

    let z = &mut [1,2,3,4,5,6,7];
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng_seed: StdRng = SeedableRng::from_seed(seed);
    println!("{:?}", z);
    rng_seed.shuffle(z);
    println!("{:?}", z);

    let a = &mut [1,2,3,4,5,6,7];
    let seed2: &[_] = &[1, 2, 3, 4, 5];
    println!("{:?}", a);
    a.shuffle_seed(seed2);
    println!("{:?}", a);

}
