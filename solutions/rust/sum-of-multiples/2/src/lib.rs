use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .map(|x|{*x})
        .filter(|x|{
           return  *x>0;
        })
        .map(|x|{
            let m = (1..)
            .map_while(|xx|{
               if xx * x < limit {
                    return Some(xx * x)
                }    
                return None ;
            })
            .collect::<HashSet<u32>>();
            return m
        })
    .flatten()
    .collect::<HashSet<u32>>()
    .iter()
    .sum::<u32>()
}