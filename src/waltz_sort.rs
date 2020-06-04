pub fn eytzinger_waltz_sort<T>(arr: &mut [T]) where T: PartialOrd {
    let count = arr.len();
    if count < 2 { return }
    'outer: for index in 0..count {
        let mut left_pointer = index * 2 + 1;
        let mut left_index = left_pointer;
        if count <= left_pointer { return }
        let mut left_level = 1;
        let mut left_start = 0usize;
        let mut right_pointer = index * 2 + 2;
        let right_index = right_pointer;
        if count <= right_pointer {
            if arr[index] < arr[left_pointer] {
                arr.swap(index, left_pointer);
            }
            return
        }
        let mut right_level = 1;
        let mut right_start = 0usize;
        let mut left_is_done = false;
        loop {
            // 2 1 3
            if arr[index] >= arr[left_pointer] && arr[index] <= arr[right_pointer] {
                if left_is_done || next_pointer(count, &mut left_pointer, &mut left_level, &mut left_start, left_index) {
                    left_is_done = true;
                    if next_pointer(count, &mut right_pointer, &mut right_level, &mut right_start, right_index) {
                        continue 'outer
                    }
                }
                continue
            }

            if arr[left_pointer] > arr[right_pointer] {
                arr.swap(left_pointer, right_pointer);
            }

            if arr[left_pointer] > arr[index] {
                arr.swap(index, left_pointer);
            } else if arr[right_pointer] < arr[index] {
                arr.swap(right_pointer, index);
                left_is_done = false;
                left_pointer = index * 2 + 1;
                left_index = left_pointer;
                left_level = 1;
            }
        }
    }
}

fn next_pointer(count: usize, pointer: &mut usize, level: &mut usize, start: &mut usize, root: usize) -> bool {
    if *pointer == root {
        let new_pointer = root * 2 + 1;
        return if new_pointer < count {
            *pointer = new_pointer;
            *level = 1;
            *start = root * 2 + 1;
            false
        } else {
            true
        };
    }
    let end =  (*start) + (1 << *level) - 1;
    if *pointer < end {
        let new_pointer = *pointer + 1;
        return if new_pointer < count {
            *pointer = new_pointer;
            false
        } else {
            true
        };
    } else {
        let new_pointer = (*start) * 2 + 1;
        return if new_pointer < count {
            *pointer = new_pointer;
            *level += 1;
            *start = (*start) * 2 + 1;
            false
        } else {
            true
        };
    }
}