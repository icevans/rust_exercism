#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + PartialOrd>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

/// Checks whether `a` is a sublist of `b`. An empty list is trivially
/// a sublist of any other list.
fn is_sublist<T: PartialEq + PartialOrd>(a: &[T], b: &[T]) -> bool {
    if a.is_empty() {
        return true;
    }

    b.windows(a.len()).any(|window| a == window)
}
