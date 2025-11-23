use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .copied()
        .filter(|x| *x > 0)
        .map(|x| {
            (1..)
                .map_while(|xx| {
                    // if xx * x < limit {
                    //     return Some(xx * x)
                    // }
                    // None
                    Some(xx * x).filter(|y| *y < limit)
                })
                .collect::<HashSet<u32>>()
        })
        .flatten()
        .collect::<HashSet<u32>>()
        .iter()
        .sum::<u32>()
}
