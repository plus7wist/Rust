fn _partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}
fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = _partition(arr, lo, hi);
        _quick_sort(arr, lo, p - 1);
        _quick_sort(arr, p + 1, hi);
    }
}
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

#[cfg(test)]
mod tests {
    use super::super::tests::is_sorted;
    use super::*;

    #[test]
    fn test_quick_sort() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        quick_sort(&mut ve1);
        assert!(is_sorted(&ve1));

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        quick_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }
}
