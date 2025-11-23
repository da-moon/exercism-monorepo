pub fn verse(n: i32) -> String {
    let mut result: String = String::new();
    if n == 0 {
        result.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    if n == 1 {
        result.push_str("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }
    if n == 2 {
        result.push_str("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    }
    if n >= 3 && n <= 99 {
        result.push_str(n.to_string().as_str());
        result.push_str(" bottles of beer on the wall, ");
        result.push_str(n.to_string().as_str());
        result.push_str(" bottles of beer.\n");
        result.push_str("Take one down and pass it around, ");
        result.push_str((n - 1).to_string().as_str());
        result.push_str(" bottles of beer on the wall.\n");
    }
    result
}

pub fn sing(start: i32, end: i32) -> String {
    let mut result: String = String::new();
    if start >= end {
        for i in (end..=start).rev() {
            result.push_str(verse(i).as_str());
            if i != end {
                result.push_str("\n");
            }
        }
    }
    result
}
