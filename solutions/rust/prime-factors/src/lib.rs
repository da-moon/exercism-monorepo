// use std::cmp;
pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut divisor = 2;
    loop {
        if n == 1 || n < divisor {
            break;
        }
        if n % divisor == 0 {
            result.push(divisor);
            n = n / divisor;
        } else {
            divisor += 1;
        }
    }
    result
}
