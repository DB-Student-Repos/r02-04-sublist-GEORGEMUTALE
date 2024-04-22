#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    if first == second {
        Comparison::Equal
    } else if first.len() > second.len() && contains_sublist(first, second) {
        Comparison::Superlist
    } else if first.len() < second.len() && contains_sublist(second, first) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

 pub fn contains_sublist<T: PartialEq>(list: &[T], sublist: &[T]) -> bool {
    if sublist.is_empty() {
        return true; 
    }
    list.windows(sublist.len()).any(|window| window == sublist)
}

