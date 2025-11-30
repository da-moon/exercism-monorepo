pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if garden.is_empty() {
        return result;
    }
    for i in 0..garden.len() {
        let mut row: String = String::new();
        for j in 0..garden[0].len() {
            if is_flower(garden, i as i32, j as i32) {
                row.push_str("*");
            } else {
                let count = count_flowers(garden, i as i32, j as i32);
                if count == 0 {
                    row.push_str(" ");
                } else {
                    let count_string = count.to_string();
                    row.push_str(count_string.as_str());
                }
            }
        }
        // XXX: why &result.push() would lead to a linter warning that it is an unused borrow ?  is it because push takes &mut self in its signature ?
        result.push(row);
    }
    return result;
}
pub fn within_bounds(garden: &[&str], r: i32, c: i32) -> bool {
    let cond = c >= 0 && r >= 0;
    let cond = cond && (r as usize) < garden.len();
    let cond = cond && (c as usize) < garden[garden.len() - 1].len();
    return cond;
}
pub fn is_flower(garden: &[&str], r: i32, c: i32) -> bool {
    let cond = within_bounds(garden, r, c);
    if cond {
        return garden[r as usize].as_bytes()[c as usize] == b'*';
    }

    return false;
}

pub fn count_flowers(garden: &[&str], i: i32, j: i32) -> u32 {
    let mut counter: u32 = 0;
    let offsets: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for offset in offsets {
        let x: i32 = i + offset.0;
        let y: i32 = j + offset.1;
        if is_flower(garden, x, y) {
            counter = counter + 1;
        }
    }
    return counter;
}
