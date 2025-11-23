use std::cmp;
pub fn factors(n: u64) -> Vec<u64> {
    let mut result : Vec<u64> = Vec::new();
    let mut i = 1;
    let upper_limit = cmp::max((n as f64).sqrt() as u64 + 1, n/10u64.pow(n.to_string().len() as u32/2));
    while  i <= upper_limit {
        println!("i {:?}",i );
        if is_prime(i) && n%i == 0{
            let mut counter : u32 = 1;
            while i.pow(counter)<= n{
                if n%i.pow(counter) == 0{
                    result.push(i)
                }
                counter += 1;
                }
            }
        i+=1;
    }
    result
}
fn is_prime(n:u64) -> bool {
    if n == 2 {
        true
    } else if n % 2 == 0 || n == 1{
        false
    } else{
        let mut result :bool = true;
        for i in 2..(n as f64).sqrt() as u64+2{
            if n % i == 0   {
                result = false;
                break;
            }
        }
        result
    }

}