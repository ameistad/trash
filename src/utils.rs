use std::collections::HashSet;

pub fn dedup_vec<T>(vs: &Vec<T>) -> Vec<T> where T: std::clone::Clone + std::cmp::Eq + std::hash::Hash {
    vs.iter()
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
