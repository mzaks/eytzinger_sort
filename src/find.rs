pub fn first_index_for_eytzinger<T>(arr: &[T], value: &T) -> Option<usize> where T: PartialOrd {
    let mut index = 0;
    let count = arr.len();
    while index < count {
        let candidate = unsafe{ arr.get_unchecked(index) };
        if value == candidate {
            return Some(index)
        }
        index = index * 2 + 1 + ((candidate < value) as usize);
    }
    None
}