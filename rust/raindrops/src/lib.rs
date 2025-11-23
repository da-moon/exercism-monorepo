pub fn raindrops(n: usize) -> String {
    let mut result : String = String::new() ;
    if n%3 == 0{
        result.push_str("Pling");
    } 
    if n%5 == 0{
        result.push_str("Plang");
    }
    if n%7 == 0{
        result.push_str("Plong");
    }
    if n%3 != 0 && n%5 != 0 && n%7 != 0 {
        result.push_str(n.to_string().as_str()) ;
    }
    result
}
