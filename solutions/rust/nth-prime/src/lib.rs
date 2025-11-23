pub fn nth(n: u32) -> Option<u32> {
       let mut result : Option<u32> = None;
    if n==1{
        result = Some(2);

    }else if n==2{
        result = Some(3);
    }else if n>2{
        let mut counter : u32 = n - 2;
        let mut iterator : u32 = 4;

        while counter>0{
            let i : Option<u32> = counter.checked_sub(1);
            if i.is_none() == false{
                if isPrime(iterator) {
                    result = Some(iterator);
                    counter =i.unwrap();
                }
                iterator+=1;
           }
        }
    }
    result
}
fn isPrime(n:u32) -> bool {
    if n == 2 {
        true
    } else if n % 2 == 0 || n == 1{
        false
    } else{
        let mut result :bool = true;
        let upperLimit = (n as f32).sqrt() as u32 + 1;
        for i in 2..upperLimit+1{
            if (n % i == 0)  {
                result = false;
                break;
            }
        }
        result
    }

}