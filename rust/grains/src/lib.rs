pub fn square(s: u32) -> u64 {
    assert!(s > 0, "Square must be between 1 and 64");
    assert!(s <= 64, "Square must be between 1 and 64");
    u64::pow(2, s-1)
}

pub fn total() -> u64 {
   (u128::pow(2u128, 64) -1).try_into().unwrap()
}
