use blake2::{Blake2b, Blake2s, Digest};

#[test]
fn blake2s_rounds() {
    let input = "test".as_bytes();
    let mut hasher = Blake2s::with_rounds(10);
    hasher.update(input);
    let rounds_res = hasher.finalize();

    let mut hasher = Blake2s::new();
    hasher.update(input);
    let res = hasher.finalize();

    assert_eq!(rounds_res, res);
}

#[test]
fn blake2b_rounds() {
    let input = "test".as_bytes();
    let mut hasher = Blake2b::with_rounds(12);
    hasher.update(input);
    let rounds_res = hasher.finalize();

    let mut hasher = Blake2b::new();
    hasher.update(input);
    let res = hasher.finalize();

    assert_eq!(rounds_res, res);
}
