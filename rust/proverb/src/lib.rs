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

pub fn build_proverb(list: &Vec<&str>) -> String {
    if list.is_empty(){
        return String::new();
    }
    // list
    // .windows(2)
    // .map(|s|
    //     vec!(s[0].to_string(),s[1].to_string())
    // )
    // .reduce(|cur: Vec<String>,nxt : Vec<String>|{
    //     vec!(
    //         format!("For want of a {} the {} was lost.",cur[0],cur[1]),
    //         format!("For want of a {} the {} was lost.",nxt[0],nxt[1]),
    //     )
    // });

    // let mut res: Vec<String> =   list
    //     .windows(2)
    //     .map(|s| vec![s[0].to_string(), s[1].to_string()])
    //     .scan(Vec::new(), |accum :&mut Vec<String>, value:Vec<String>| {
    //         let sentence: String = format!("For want of a {} the {} was lost.", value[0], value[1]);
    //         Some(sentence)
    //     })
    //     .collect::<Vec<String>>();
    
    let mut res: Vec<String> =   list
        .windows(2)
        .map(|s| vec![s[0].to_string(), s[1].to_string()])
        .fold(Vec::new(), |cur: Vec<String>, nxt: Vec<String>| {
            let sentence: String = format!("For want of a {} the {} was lost.", nxt[0], nxt[1]);
            [cur, vec![sentence]].concat()
        });
    let sentence: String = format!("And all for the want of a {}.",list[0]);
    res.push(sentence);
    return res.join("\n");
}