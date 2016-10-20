use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Debug + Sized>(v1: &[T], v2: &[T]) -> Comparison {
    match v1.len().cmp(&v2.len()) {
        Ordering::Equal => compare(v1, v2),
        Ordering::Less => {
            if v1.is_empty() {
                return Comparison::Sublist;
            }

            let firstOfV1 = &v1[0];
            if v2.iter().enumerate().any(|(i, elementOfV2)| {
                if firstOfV1 == elementOfV2 {
                    match compare(v1, &v2[i..]) {
                        Comparison::Equal => return true,
                        _ => return false,
                    }
                }

                false
            }) {
                return Comparison::Sublist;
            }

            Comparison::Unequal
        }
        Ordering::Greater => {
            if Comparison::Sublist == sublist(v2, v1) {
                return Comparison::Superlist;
            }

            Comparison::Unequal
        }
    }
}

fn compare<T: PartialEq + Debug + Sized>(v1: &[T], v2: &[T]) -> Comparison {
    if v1.len() > v2.len() {
        return Comparison::Unequal;
    }

    if v1.iter().zip(v2.iter()).all(|(x, y)| x == y) {
        return Comparison::Equal;
    }

    Comparison::Unequal
}
