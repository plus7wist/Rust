use std::cmp::Ordering;

pub trait MergeSort<T: Ord> {
    fn sort_by<F>(&mut self, compare: F)
        where F: FnMut(&T, &T) -> Ordering;
    // fn sort(&mut self) -> Ordering;
}

impl<T> MergeSort for [T] {
    fn sort_by(&mut self, compare: F)
        where F: FnMut(&T, &T) -> Ordering
    {
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::is_sorted;
    use super::*;

    // #[test]
    // fn merge_sort_simple1() {
    //     let mut v = vec![10, 13, 14, 12, 15];
    //     merge_sort(&mut v);
    //     assert!(is_sorted(&v));
    // }

    // #[test]
    // fn merge_sort_dec() {
    //     let mut v = vec![10, 9, 8, 7, 6, 5];
    //     merge_sort(&mut v);
    //     assert!(is_sorted(&v));
    // }

    // #[test]
    // fn merge_sort_inc() {
    //     let mut v = vec![5, 6, 7, 8, 9, 10];
    //     merge_sort(&mut v);
    //     assert!(is_sorted(&v));
    // }

    // use std::cmp::Ordering;
    // use std::sync::Mutex;

    // struct NoClone {
    //     data: Mutex<i32>,
    // }

    // impl NoClone {
    //     fn new(data: i32) -> Self {
    //         Self {
    //             data: Mutex::new(data),
    //         }
    //     }

    //     fn get(&self) -> i32 {
    //         *self.data.lock().unwrap()
    //     }
    // }

    // impl PartialEq for NoClone {
    //     fn eq(&self, other: &NoClone) -> bool {
    //         self.get() == other.get()
    //     }
    // }

    // impl PartialOrd for NoClone {
    //     fn partial_cmp(&self, other: &NoClone) -> Option<Ordering> {
    //         self.get().partial_cmp(&other.get())
    //     }
    // }

    // impl Eq for NoClone {}

    // impl Ord for NoClone {
    //     fn cmp(&self, other: &NoClone) -> Ordering {
    //         self.partial_cmp(other).unwrap()
    //     }
    // }

    // #[test]
    // fn merge_sort_no_clone() {
    //     let mut v = vec![NoClone::new(5), NoClone::new(4), NoClone::new(3)];
    //     merge_sort(&mut v);
    //     assert!(is_sorted(&v));
    // }
}
