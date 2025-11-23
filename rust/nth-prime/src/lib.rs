// pub fn nth(n: u32) -> u32 {
//     let mut primes : Vec<u32> = vec![2,3];
//     // NOTE: in case n == 0 or n==1 , then there is no need to check for prime value. we can simply return primes[n]
//     let mut iterations : u32 = 0 ;
//     if (primes.len() as u32) <= n {
//         iterations = (n - primes.len() as u32) +1 ;
//     }
//     // NOTE: initializing variable that will hold the number we are testing to see if it is prime or not
//     let mut curr : u32 = 0 ;
//     while iterations > 0 {
//         let mut is_prime = true;
//         // NOTE: get the number greater than the last element in primes array
//         if *primes.last().unwrap() + 1 > curr {
//             curr = *primes.last().unwrap() + 1 ;
//         }
//         // NOTE: iterate over primes array
//         for i in primes.iter() {
//             if curr % i == 0 { is_prime = false ; break ; } ;
//         }
//         if is_prime {
//             iterations -=1 ;
//             primes.push(curr);
//             continue ;
//         }
//         curr +=1 ;
//     }
//     return *primes.index(usize::try_from(n).unwrap());
// }
pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::with_capacity((n as usize) + 1);

    (2..)
        .filter(|candidate| {
            if !primes.iter().any(|i| candidate % i == 0) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
