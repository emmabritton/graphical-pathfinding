use std::rc::Rc;

pub trait RcBreaker<T> {
    fn contains_item(&self, target: &T) -> bool where T: PartialEq<T> + PartialEq<Rc<T>>;
}

impl<T> RcBreaker<T> for Vec<Rc<T>> {
    fn contains_item(&self, target: &T) -> bool where T: PartialEq<T> + PartialEq<Rc<T>> {
        for item in self {
            if target == item {
                return true;
            }
        }
        return false;
    }
}

pub fn max(lhs: f64, rhs: f64) -> f64 {
    if lhs < rhs {
        rhs
    } else {
        lhs
    }
}