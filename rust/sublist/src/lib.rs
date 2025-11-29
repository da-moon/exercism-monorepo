#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let mut shorter = first_list;
    let mut longer = second_list;
    if first_list.len() > second_list.len() {
        shorter = second_list;
        longer = first_list;
    }
    let result = is_sublist(shorter, longer);
    if result == true {
        if first_list.len() == second_list.len() {
            return Comparison::Equal;
        }
        if first_list.len() < second_list.len() {
            return Comparison::Sublist;
        }
        if first_list.len() > second_list.len() {
            return Comparison::Superlist;
        }
    }

    return Comparison::Unequal;

    // todo!(
    //     "Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}."
    // );
}
fn is_sublist(shorter: &[i32], longer: &[i32]) -> bool {
    if shorter.is_empty() {
        return true;
    }
    longer.windows(shorter.len()).any(|x| x == shorter)
}
