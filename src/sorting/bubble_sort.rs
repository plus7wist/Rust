pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::is_sorted;
    use super::*;

    #[test]
    fn bubble_sort_descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut ve1);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn bubble_sort_pre_sorted() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }
}
