#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    fn within<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        b.windows(a.len()).any(|x| a == x)
    }
    if first_list == second_list {
        return Comparison::Equal;
    }
    if first_list.is_empty() || within(first_list, second_list) {
        return Comparison::Sublist;
    }
    if second_list.is_empty() || within(second_list, first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
