use std::collections::HashSet;

pub fn dedup_vec<T>(vs: &Vec<T>) -> Vec<T> where T: std::clone::Clone + std::cmp::Eq + std::hash::Hash {
    vs.iter()
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup_vec() {
        let vec1 = dedup_vec(&vec![1, 1, 2, 3]);
        let vec2 = dedup_vec(&vec![1, 2, 3]);
        assert_eq!(vec1.len(), vec2.len());
    }
}
