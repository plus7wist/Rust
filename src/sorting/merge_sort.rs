pub fn merge_sort<T: Ord>(array: &mut [T]) {
    let mut index = vec![0usize; array.len()];
    let mut buffer = vec![0usize; array.len()];
    merge_sort_conquer(array, &mut index, &mut buffer);
    rearrange(array, &mut index, &mut buffer);
}

fn rearrange<T>(array: &mut [T], index: &mut [usize], buffer: &mut [usize]) {
    let n = array.len();

    for i in 0..n {
        buffer[index[i]] = i;
    }

    for i in 0..n {
        array.swap(i, index[i]);
        index[buffer[i]] = index[i];
    }
}

fn merge_sort_merge<T: Ord>(
    array: &mut [T],
    index: &mut [usize],
    bufferl: &mut [usize],
    bufferr: &mut [usize],
) {
    let mut lt = bufferl.iter().peekable();
    let mut rt = bufferr.iter().peekable();
    let mut it = index.iter_mut();

    while let (Some(l), Some(r)) = (lt.peek(), rt.peek()) {
        *it.next().unwrap() = if array[**l] < array[**r] {
            *lt.next().unwrap()
        } else {
            *rt.next().unwrap()
        };
    }

    while let Some(l) = lt.next() {
        *it.next().unwrap() = *l;
    }

    while let Some(r) = rt.next() {
        *it.next().unwrap() = *r;
    }
}

// Make `index` be the index order of sorted `array`.
fn merge_sort_conquer<T: Ord>(array: &mut [T], index: &mut [usize], buffer: &mut [usize]) {
    match array.len() {
        0 => {}
        1 => {
            index[0] = 0;
            buffer[0] = 0;
        }
        n => {
            let middle = n / 2;
            let (bufferl, bufferr) = buffer.split_at_mut(middle);
            let (indexl, indexr) = index.split_at_mut(middle);
            let (arrayl, arrayr) = array.split_at_mut(middle);

            merge_sort_conquer(arrayl, bufferl, indexl);
            merge_sort_conquer(arrayr, bufferr, indexr);
            bufferr.iter_mut().for_each(|it| *it += middle);

            merge_sort_merge(array, index, bufferl, bufferr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::is_sorted;
    use super::*;

    #[test]
    fn merge_sort_simple1() {
        let mut v = vec![10, 13, 14, 12, 15];
        merge_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn merge_sort_dec() {
        let mut v = vec![10, 9, 8, 7, 6, 5];
        merge_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn merge_sort_inc() {
        let mut v = vec![5, 6, 7, 8, 9, 10];
        merge_sort(&mut v);
        assert!(is_sorted(&v));
    }

    use std::cmp::Ordering;
    use std::sync::Mutex;

    struct NoClone {
        data: Mutex<i32>,
    }

    impl NoClone {
        fn new(data: i32) -> Self {
            Self {
                data: Mutex::new(data),
            }
        }

        fn get(&self) -> i32 {
            *self.data.lock().unwrap()
        }
    }

    impl PartialEq for NoClone {
        fn eq(&self, other: &NoClone) -> bool {
            self.get() == other.get()
        }
    }

    impl PartialOrd for NoClone {
        fn partial_cmp(&self, other: &NoClone) -> Option<Ordering> {
            self.get().partial_cmp(&other.get())
        }
    }

    impl Eq for NoClone {}

    impl Ord for NoClone {
        fn cmp(&self, other: &NoClone) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    #[test]
    fn merge_sort_no_clone() {
        let mut v = vec![NoClone::new(5), NoClone::new(4), NoClone::new(3)];
        merge_sort(&mut v);
        assert!(is_sorted(&v));
    }
}
