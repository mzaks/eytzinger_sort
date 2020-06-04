unsafe fn find_min<T>(arr: &[T]) -> usize where T: PartialOrd {
    let mut i = 0;
    for j in 1..arr.len() {
        if arr.get_unchecked(j) < arr.get_unchecked(i) {
            i = j
        }
    }
    i
}

unsafe fn find_next_bigger_then_min<T>(arr: &[T], min: usize) -> (bool, usize) where T: PartialOrd {
    let mut found = false;
    let mut i = if min == 0 {1} else {0};
    for j in 0..arr.len() {
        if (found == false || arr.get_unchecked(j) <= arr.get_unchecked(i)) && arr.get_unchecked(j) > arr.get_unchecked(min) {
            found = true;
            i = j;
        }
    }
    (found, i)
}

unsafe fn traverse<T>(arr: &mut [T], index: usize, min: &mut usize) where T: PartialOrd {
    if index >= arr.len() {
        return
    }
    let left = index * 2 + 1;
    if left < arr.len() {
        traverse(arr, left, min);
        arr.swap(index, *min);
        let next = find_next_bigger_then_min(arr, index);
        *min = next.1;
        traverse(arr, left + 1, min);
    } else {
        arr.swap(index, *min);
        let next = find_next_bigger_then_min(arr, index);
        *min = next.1;
    }
}

pub fn eytzinger_traversal_sort<T>(arr: &mut [T]) where T: PartialOrd {
    unsafe {
        let mut min = find_min(arr);
        traverse(arr, 0, &mut min);
    }
}


#[cfg(test)]
mod test {
    use crate::traversal_sort::{find_min, find_next_bigger_then_min, eytzinger_traversal_sort};

    #[test]
    fn test_find_min() {
        let arr = [9, 34675, 2342, 545, 7, 56, 13];
        unsafe {
            assert_eq!(find_min(&arr), 4);
        }
    }

    #[test]
    fn test_find_bigger_then_min() {
        let arr = [9, 34675, 2342, 545, 7, 56, 13];
        unsafe  {
            assert_eq!(find_next_bigger_then_min(&arr, 0), (true, 6));
        }
    }

    #[test]
    fn test_find_bigger_then_min2() {
        let arr = [2, 1, 3];
        unsafe  {
            assert_eq!(find_next_bigger_then_min(&arr, 0), (true, 2));
        }
    }

    #[test]
    fn test_traversal_sort() {
        let mut arr = vec![1, 2];
        eytzinger_traversal_sort(&mut arr);
        assert_eq!(arr, vec![2, 1]);

        let mut arr = vec![1, 2, 3];
        eytzinger_traversal_sort(&mut arr);
        assert_eq!(arr, vec![2, 1, 3]);

        let mut arr = vec![1, 2, 3, 4];
        eytzinger_traversal_sort(&mut arr);
        assert_eq!(arr, vec![3, 2, 4, 1]);

        let mut arr = vec![1, 2, 3, 4, 5];
        eytzinger_traversal_sort(&mut arr);
        assert_eq!(arr, vec![4, 2, 5, 1, 3]);

        let mut arr = vec![1, 2, 3, 4, 5, 6];
        eytzinger_traversal_sort(&mut arr);
        assert_eq!(arr, vec![4, 2, 6, 1, 3, 5]);

        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        eytzinger_traversal_sort(&mut arr);
        assert_eq!(arr, vec![4, 2, 6, 1, 3, 5, 7]);

        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        eytzinger_traversal_sort(&mut arr);
        assert_eq!(arr, vec![5, 3, 7, 2, 4, 6, 8, 1]);

        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        eytzinger_traversal_sort(&mut arr);
        assert_eq!(arr, vec![6, 4, 8, 2, 5, 7, 9, 1, 3]);
    }
}