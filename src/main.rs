#![allow(dead_code)]

use rand_core::RngCore;
use ssrand::RngJumpAhead;
//use rand::Rng;

fn test_new_and_next_u32() {
    //let mut s = ssrand::Cong::new(1);
    //let mut s = ssrand::SHR3::new(1);
    //let mut s = ssrand::MWC1::new(1, 2);
    //let mut s = ssrand::MWC2::new(1, 2);
    //let mut s = ssrand::KISS::new(1, 2, 3, 4);
    //let mut s = ssrand::MWC64::new(1, 2);
    //let mut s = ssrand::KISS2::new(1, 2, 3, 4);
    //let mut s = ssrand::LFSR88::new(1, 2, 3);
    let mut s = ssrand::LFSR113::new(1, 2, 3, 4);
    for _ in 0..4 {
        println!("{}, {:?}", s.next_u32(), s);
        //println!("{}, {:?}", s.gen::<u32>(), s);
        //println!("{:?}", s.gen::<(f64)>());
    }

    let jumpahead_n = 1_000_000_000_000_000_000_i64;
    s.jumpahead(jumpahead_n);
    println!("jumpahead by {}", jumpahead_n);
    println!("{}, {:?}", s.next_u32(), s);
}

fn test_maths() {
    println!(
        "wrapping_pow {}",
        ssrand::math::wrapping_pow(123456789_u32, 123456789_u32)
    );
    println!(
        "pow_mod {:#X}",
        ssrand::math::pow_mod(0xDC28D76F_usize, 0x732E73C3_usize, 0xEC327D45_usize)
    );
    println!(
        "pow_mod {:#X}",
        ssrand::math::pow_mod(
            0xDC28D76FFD9338E9D868AF566191DE10_u128,
            0x732E73C316878E244FDFDE4EE623CDCC_u128,
            0xEC327D45470669CC56B547B6FE6888A2_u128
        )
    );
    println!(
        "wrapping_geom_series {}",
        ssrand::math::wrapping_geom_series(12345_u32, 12345_u32)
    );
}

fn main() {
    test_new_and_next_u32();

    println!("");
    test_maths();
}
