use eytzinger::foundation;

pub fn clone_as_eytzinger<T>(arr: &[T]) -> Vec<T> where T: PartialOrd + Clone {
    if arr.is_empty() {
        return vec![];
    }
    let mut result = Vec::with_capacity(arr.len());
    let g = PermutationGenerator::new(arr.len());
    for index in g {
        result.push(arr[index].clone());
    }
    result
}

#[derive(Clone, Debug)]
pub struct PermutationGenerator {
    size: usize,
    ipk: usize,
    li: usize,
}

impl PermutationGenerator {
    /// Generate a new permutation for a sorted array of a given size.
    #[inline]
    pub fn new(size: usize) -> PermutationGenerator {
        PermutationGenerator {
            size,
            ipk: 1,
            li: 0,
        }
    }
}

impl Iterator for PermutationGenerator {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        let k2 = 1 << (self.ipk - 1);

        if k2 + self.li - 1 >= self.size {
            return None;
        }

        if self.li >= k2 {
            self.li = 0;
            self.ipk += 1;
        }

        let li = self.li;
        self.li += 1;
        Some(foundation::get_permutation_element_by_node(self.size, self.ipk, li))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let k2 = 1 << (self.ipk - 1);
        let size = self.size - (k2 + self.li - 1);
        (size, Some(size))
    }
}