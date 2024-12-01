use ssrand::RngJumpAhead;
use rand_core::RngCore;

#[test]
fn test_kiss_million() {
    let mut rng = ssrand::KISS::new(2247183469, 99545079, 3269400377, 3950144837);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 2100752872);

    let mut rng_ja = ssrand::KISS::new(2247183469, 99545079, 3269400377, 3950144837);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_cong_million() {
    let mut rng = ssrand::Cong::new(2051391225);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 2416584377);

    let mut rng_ja = ssrand::Cong::new(2051391225);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_shr3_million() {
    let mut rng = ssrand::SHR3::new(3360276411);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 1153302609);

    let mut rng_ja = ssrand::SHR3::new(3360276411);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_mwc1_million() {
    let mut rng = ssrand::MWC1::new(2374144069, 1046675282);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 904977562);

    let mut rng_ja = ssrand::MWC1::new(2374144069, 1046675282);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_mwc2_million() {
    let mut rng = ssrand::MWC2::new(0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 767834450);

    let mut rng_ja = ssrand::MWC2::new(0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_mwc64_million() {
    let mut rng = ssrand::MWC64::new(0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 2191957470);

    let mut rng_ja = ssrand::MWC64::new(0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_kiss2_million() {
    let mut rng = ssrand::KISS2::new(0, 0, 0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 4044786495);

    let mut rng_ja = ssrand::KISS2::new(0, 0, 0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_lfsr113_million() {
    let mut rng = ssrand::LFSR113::new(0, 0, 0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 300959510);

    let mut rng_ja = ssrand::LFSR113::new(0, 0, 0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_lfsr88_million() {
    let mut rng = ssrand::LFSR88::new(0, 0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 3774296834);

    let mut rng_ja = ssrand::LFSR88::new(0, 0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}
