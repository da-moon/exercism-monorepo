pub fn is_armstrong_number(num: u32) -> bool {
    let length  = num.to_string().len() as u32;
    let mut sum :u32 = 0;
    for i in num.to_string().chars() {
        if i.to_digit(10) != None{
            sum += (i.to_digit(10).unwrap()).pow(length);
        }
    }
    num == sum
}
