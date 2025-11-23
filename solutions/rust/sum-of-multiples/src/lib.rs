use std::collections::BTreeSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result : u32 =0;
    let mut result_set : BTreeSet<u32>= BTreeSet::new();
    for factor in factors {
        for number in 1..limit{
            if number%factor == 0{
                result_set.insert(number);
            }
        }
    }
    if result_set.len()>0 {

        for item in result_set{
            result+=item;
        }
    }
    result
}
