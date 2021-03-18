use blake2;

// https://tools.ietf.org/html/rfc7693#appendix-A
#[test]
fn blake2b_f_function() {
    let rounds = 12;
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
    let m: [u64; 16] = [
        0x0000000000636261,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ];
    let t: [u64; 2] = [3, 0];
    let f_bool = true;

    let output: [u64; 8] = [
        0x0D4D1C983FA580BA,
        0xE9F6129FB697276A,
        0xB7C45A68142F214C,
        0xD1A2FFDB6FBB124B,
        0x2D79AB2A39C5877D,
        0x95CC3345DED552C2,
        0x5A92F1DBA88AD318,
        0x239900D4ED8623B9,
    ];

    let res = blake2::blake2b_f(rounds, h, m, t, f_bool);

    assert_eq!(res, output);
}

// https://tools.ietf.org/html/rfc7693#appendix-A
#[test]
fn blake2s_f_function() {
    let rounds = 10;
    let h: [u32; 8] = [
        0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB,
        0x5BE0CD19,
    ];
    let m = [
        0x00636261, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000,
    ];
    let t: [u32; 2] = [3, 0];
    let f_bool = true;

    let output: [u32; 8] = [
        0x8C5E8C50, 0xE2147C32, 0xA32BA7E1, 0x2F45EB4E, 0x208B4537, 0x293AD69E, 0x4C9B994D,
        0x82596786,
    ];

    let res = blake2::blake2s_f(rounds, h, m, t, f_bool);

    assert_eq!(res, output);
}
