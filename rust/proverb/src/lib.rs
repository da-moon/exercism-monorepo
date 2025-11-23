// pub fn build_proverb(list: &[&str]) -> String {
//     list.windows(2)
//         .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
//         .chain(
//             list.first()
//                 .map(|&first| format!("And all for the want of a {}.", first))
//                 .into_iter(),
//         )
//         .collect::<Vec<_>>()
//         .join("\n")
// }
// pub fn build_proverb(list: &[&str]) -> String {
//     let body = list.windows(2).fold(String::new(), |mut acc, w| {
//         acc.push_str(&format!("For want of a {} the {} was lost.\n", w[0], w[1]));
//         acc
//     });
//     list.first().map_or(String::new(), |first| {
//         format!("{}And all for the want of a {}.", body, first)
//     })
// }

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut lines: Vec<String> = list
        .windows(2)
        .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
        .collect();

    lines.push(format!("And all for the want of a {}.", list[0]));
    lines.join("\n")
}
