// pub fn brackets_are_balanced(string: &str) -> bool {
//     if string.len() == 0 {
//         return true;
//     }
//     let col = string
//         .chars()
//         .into_iter()
//         // filter out brackets
//         .filter(|x| {
//             match x {
//                 '[' | '(' | '{' | '}' | ')' | ']' => {
//                     return true;
//                 }
//                 _ => {
//                     return false;
//                 }
//             };
//         })
//         .scan(
//             (Vec::new(), true),
//             |state: &mut (Vec<char>, bool), curr: char| {
//                 match curr {
//                     '[' | '(' | '{' => {
//                         state.0.push(curr);
//                     }
//                     ']' | ')' | '}' => {
//                         if let Some(last) = state.0.pop(){
//                             // in case brackets don't match
//                             if 
//                             ( last == '[' && curr != ']' ) ||
//                             ( last == '{' && curr != '}' ) ||
//                             ( last == '(' && curr != ')' )
//                              {
//                                 state.1 = false;
//                             }
//                         }else{
//                             // if stack is empty
//                             state.1=false;
//                         }
//                     }
//                     _ => (),
//                 };
//                Some((*state).clone())
//             },
//         )
//         .last()
//         .unwrap();
//         return col.1 && col.0.len() == 0 ;
// }
pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .try_fold(vec![], |mut stack, ch| {
            match ch {
                '{' | '[' | '(' => stack.push(ch),
                '}' => {
                    if stack.pop() != Some('{') {
                        return Err(());
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return Err(());
                    }
                }
                ')' => {
                    if stack.pop() != Some('(') {
                        return Err(());
                    }
                }
                _ => (),
            }
            Ok(stack)
        })
        .map(|stack| stack.is_empty())
        .unwrap_or(false)
}