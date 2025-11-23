pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len()+1];
    }
    digits
    .chars()
    .map(|x|{
        x.to_string()
    })
    .collect::<Vec<String>>()
    .windows(len)
    .map(|s| {
        (0..len)
        .fold("".to_string(),|accum: String, n:usize|{
            format!("{}{}",accum,s[n].to_string())
        })
    })
    .collect::<Vec<String>>()
}
