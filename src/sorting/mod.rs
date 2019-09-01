mod bubble_sort;
mod counting_sort;
mod heap_sort;
mod insertion_sort;
mod quick_sort;
mod selection_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::counting_sort::counting_sort;
pub use self::counting_sort::generic_counting_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::quick_sort::quick_sort;
pub use self::selection_sort::selection_sort;

// Help functions used in tests.
#[cfg(test)]
mod tests {
    use std::cmp;

    pub fn is_sorted<T>(arr: &[T]) -> bool
    where
        T: cmp::PartialOrd,
    {
        if arr.is_empty() {
            return true;
        }

        let mut prev = &arr[0];

        for item in arr.iter().skip(1) {
            if prev > &item {
                return false;
            }

            prev = &item;
        }

        true
    }
}
