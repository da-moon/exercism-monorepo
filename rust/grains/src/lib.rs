pub fn square(s: u32) -> u64 {
    if s > 0 && s < 65 {
        2u64.pow(s - 1)
    } else {
        panic!("Square must be between 1 and 64");
    }
}

pub fn total() -> u64 {
    let mut result: u64 = 0;
    for i in 1..=64 {
        result += square(i);
    }
    result
}
