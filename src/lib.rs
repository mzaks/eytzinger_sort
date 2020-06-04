extern crate eytzinger;
extern crate rand;

pub mod find;
pub mod march_sort;
pub mod traversal_sort;
pub mod waltz_sort;
pub mod clone;

#[cfg(test)]
mod tests {
    use crate::clone::clone_as_eytzinger;
    use crate::march_sort::eytzinger_march_sort;
    use crate::waltz_sort::eytzinger_waltz_sort;
    use crate::traversal_sort::eytzinger_traversal_sort;
    use crate::find::first_index_for_eytzinger;

    #[test]
    fn validate_march_sort() {
        for i in 2..100 {
            let mut arr: Vec<usize> = (1..=i).collect();
            eytzinger_march_sort(&mut arr);
            for (index, e) in arr.iter().enumerate() {
                assert_eq!(first_index_for_eytzinger(&arr, e).expect("March sort has a problem"), index)
            }
        }
    }

    #[test]
    fn validate_waltz_sort() {
        for i in 2..100 {
            let mut arr: Vec<usize> = (1..=i).collect();
            eytzinger_waltz_sort(&mut arr);
            for (index, e) in arr.iter().enumerate() {
                assert_eq!(first_index_for_eytzinger(&arr, e).expect("Waltz sort has a problem"), index)
            }
        }
    }

    #[test]
    fn validate_traversal_sort() {
        for i in 2..100 {
            let mut arr: Vec<usize> = (1..=i).collect();
            eytzinger_traversal_sort(&mut arr);
            for (index, e) in arr.iter().enumerate() {
                assert_eq!(first_index_for_eytzinger(&arr, e).expect("Traversal sort has a problem"), index)
            }
        }
    }

    #[test]
    fn validate_clone() {
        for i in 2..100 {
            let arr: Vec<usize> = (1..=i).collect();
            let arr = clone_as_eytzinger(&arr);
            for (index, e) in arr.iter().enumerate() {
                assert_eq!(first_index_for_eytzinger(&arr, e).expect("Traversal sort has a problem"), index)
            }
        }
    }

    #[test]
    fn duplicates_and_march_sort() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        eytzinger_march_sort(&mut arr);
        assert_eq!(arr, [5, 3, 7, 2, 4, 6, 8, 1]);

        let mut arr = [4, 2, 3, 4, 5, 6, 7, 8];
        eytzinger_march_sort(&mut arr);
        assert_eq!(arr, [5, 4, 7, 3, 4, 6, 8, 2]);

        let mut arr = [4, 2, 3, 4, 5, 6, 7, 4];
        eytzinger_march_sort(&mut arr);
        assert_eq!(arr, [4, 4, 6, 3, 4, 5, 7, 2]);

        let mut arr = [4, 2, 3, 4, 5, 6, 4, 4];
        eytzinger_march_sort(&mut arr);
        assert_eq!(arr, [4, 4, 5, 3, 4, 4, 6, 2]);

        let mut arr = vec![4, 1, 1, 1, 12, 1, 1, 1, 5, 1, 1, 1];
        eytzinger_march_sort(&mut arr);
        assert_eq!(arr, [1, 1, 5, 1, 1, 4, 12, 1, 1, 1, 1, 1]);
    }
}
