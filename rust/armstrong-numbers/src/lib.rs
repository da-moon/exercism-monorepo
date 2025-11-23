pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let num_str = num.to_string();
    let exp : u32 = num_str.len() as u32;
    let mut sum : u32 = 0 ;
    for c in num_str.chars() {
        if let Some(n) = c.to_digit(10) {
            if let Some(val) = sum.checked_add(n.pow(exp)) {
                sum = val ;
            }
        }
    }
    sum == num

    // let exp = num.ilog10()+1 ;
    // let mut dividend = num  ;
    // let mut sum = 0 ;
    // // https://stackoverflow.com/a/31642585
    // for i in (0..exp).rev() {
    //     let divisor = 10_u32.pow(i) ;
    //     let quotient = dividend / divisor ;
    //     dividend = dividend % divisor ;
    //     sum += quotient.pow(exp);
    // } ;
    // sum == num

}
