pub fn eytzinger_march_sort<T>(arr: &mut [T]) where T:PartialOrd {
    let count = arr.len();
    if count < 2 { return }
    'outer: for index in 0..count {
        let mut first = true;
        loop {
            let left_subtree_root_index = index * 2 + 1;
            if count <= left_subtree_root_index { return }
            let index_of_largest_left_item = find_index_of_largest_subtree_item(arr, left_subtree_root_index);
            unsafe {
                if arr.get_unchecked(index) > arr.get_unchecked(index_of_largest_left_item) {
                    if first == false { continue 'outer };
                } else {
                    arr.swap(index, index_of_largest_left_item);
                }
            }
            first = false;
            let right_subtree_root_index = index * 2 + 2;
            if count <= right_subtree_root_index { return }
            let index_of_smallest_right_item = find_index_of_smallest_subtree_item(arr, right_subtree_root_index);
            unsafe {
                if arr.get_unchecked(index) <= arr.get_unchecked(index_of_smallest_right_item) {
                    continue 'outer;
                } else {
                    arr.swap(index, index_of_smallest_right_item);
                }
            }
        }
    }
}

#[inline]
fn find_index_of_largest_subtree_item<T>(arr: &[T], index: usize) -> usize where T: PartialOrd {
    let mut candidate = index;
    let mut level = 1;
    let mut start = index * 2 + 1;
    loop {
        let end = start + (1 << level);
        unsafe {
            for i in start..end {
                if arr.len() <= i { return candidate }
                if arr.get_unchecked(candidate) < arr.get_unchecked(i) {
                    candidate = i;
                }
            }
        }
        start = start * 2 + 1;
        level += 1
    }
}

#[inline]
fn find_index_of_smallest_subtree_item<T>(arr: &[T], index: usize) -> usize where T: PartialOrd {
    let mut candidate = index;
    let mut level = 1;
    let mut start = index * 2 + 1;
    loop {
        let end = start + (1 << level);
        unsafe {
            for i in start..end {
                if arr.len() <= i { return candidate }
                if arr.get_unchecked(candidate) > arr.get_unchecked(i) {
                    candidate = i;
                }
            }
        }
        start = start * 2 + 1;
        level += 1
    }
}