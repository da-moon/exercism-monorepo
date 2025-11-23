pub fn square_of_sum(n: usize) -> usize {
    let mut result : usize = 0;
    for i in 1..=n{
        result+=i;
    }
    result *=result;
    result
}

pub fn sum_of_squares(n: usize) -> usize {
    let mut result : usize = 0;
    for i in 1..=n{
        result+=i*i;
    }
    result
}

pub fn difference(n: usize) -> usize {
    let mut result : usize = 0;
    let i : Option<usize> = square_of_sum(n).checked_sub(sum_of_squares(n));
    if i.is_none() == false{
        result  = i.unwrap();
    }
    result
}
