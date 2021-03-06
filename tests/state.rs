// https://tools.ietf.org/html/rfc7693#appendix-A
#[test]
fn blake2b_state() {
    let rounds = 12;
    // Initial IV with parameter block.
    let h: [u64; 8] = [
        0x6a09e667f2bdc948,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];
    let m = b"abc";
    let t0 = 0;
    let t1 = 0;
    let f0 = !0;
    let f1 = 0;

    let expected: [u8; 64] = [
        0xba, 0x80, 0xa5, 0x3f, 0x98, 0x1c, 0x4d, 0x0d, 0x6a, 0x27, 0x97, 0xb6, 0x9f, 0x12, 0xf6,
        0xe9, 0x4c, 0x21, 0x2f, 0x14, 0x68, 0x5a, 0xc4, 0xb7, 0x4b, 0x12, 0xbb, 0x6f, 0xdb, 0xff,
        0xa2, 0xd1, 0x7d, 0x87, 0xc5, 0x39, 0x2a, 0xab, 0x79, 0x2d, 0xc2, 0x52, 0xd5, 0xde, 0x45,
        0x33, 0xcc, 0x95, 0x18, 0xd3, 0x8a, 0xa8, 0xdb, 0xf1, 0x92, 0x5a, 0xb9, 0x23, 0x86, 0xed,
        0xd4, 0x0, 0x99, 0x23,
    ];

    let mut hasher = near_blake2::VarBlake2b::with_state(rounds, h, [t0, t1]);
    hasher.update_inner(m);
    hasher.compress(f0, f1);
    let res = hasher.output();

    assert_eq!(res.as_slice(), expected);
}

// https://tools.ietf.org/html/rfc7693#appendix-A
#[test]
fn blake2s_state() {
    let rounds = 10;
    let h: [u32; 8] = [
        0x6b08e647, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let m: &[u8; 3] = b"abc";
    let t0 = 0;
    let t1 = 0;
    let f0 = !0;
    let f1 = 0;

    let expected: &[u8; 32] = &[
        0x50, 0x8c, 0x5e, 0x8c, 0x32, 0x7c, 0x14, 0xe2, 0xe1, 0xa7, 0x2b, 0xa3, 0x4e, 0xeb, 0x45,
        0x2f, 0x37, 0x45, 0x8b, 0x20, 0x9e, 0xd6, 0x3a, 0x29, 0x4d, 0x99, 0x9b, 0x4c, 0x86, 0x67,
        0x59, 0x82,
    ];

    let mut hasher = near_blake2::VarBlake2s::with_state(rounds, h, [t0, t1]);
    hasher.update_inner(m);
    hasher.compress(f0, f1);
    let res = hasher.output();

    assert_eq!(res.as_slice(), expected);
}

#[test]
fn blake2b_state_100r() {
    let rounds = 100;
    // Initial IV with parameter block.
    let h: [u64; 8] = [
        0x6a09e667f2bdc948,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];
    let m = b"abc";
    let t0 = 0;
    let t1 = 0;
    let f0 = !0;
    let f1 = 0;

    let expected: [u8; 64] = [
        0x81, 0x36, 0x97, 0x2e, 0xb0, 0xcc, 0x5, 0xf3, 0xdf, 0x5e, 0xc9, 0x8a, 0xc6, 0x4b, 0x7e,
        0x95, 0x57, 0xba, 0xd1, 0x47, 0x11, 0xf1, 0x5f, 0xde, 0x8f, 0x42, 0x83, 0x4c, 0xf6, 0xbb,
        0xab, 0x48, 0x1b, 0x81, 0xb1, 0xb3, 0xc0, 0xbd, 0x70, 0x3f, 0x65, 0x69, 0x4e, 0x76, 0x39,
        0x75, 0xd0, 0x5c, 0x55, 0xc7, 0x31, 0xbf, 0xa1, 0xaf, 0xdd, 0x3a, 0x11, 0x8d, 0xcb, 0xf4,
        0xef, 0x7c, 0x28, 0x45,
    ];

    let mut hasher = near_blake2::VarBlake2b::with_state(rounds, h, [t0, t1]);
    hasher.update_inner(m);
    hasher.compress(f0, f1);
    let res = hasher.output();

    assert_eq!(res.as_slice(), expected);
}
