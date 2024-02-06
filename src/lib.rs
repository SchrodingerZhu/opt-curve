use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn forward_distance<T: PartialEq + Eq + Clone + Hash>(trace: &[T]) -> Vec<usize> {
    let mut last_occurence = HashMap::new();
    let mut result = vec![0; trace.len()];
    for (i, element) in trace.iter().enumerate().rev() {
        match last_occurence.entry(element) {
            Entry::Occupied(mut x) => {
                result[i] = x.get() - i;
                *x.get_mut() = i;
            }
            Entry::Vacant(x) => {
                result[i] = usize::MAX;
                x.insert(i);
            }
        }
    }
    result
}

// pub fn evicted_by_rule_omega<T: PartialEq + Eq + Clone>(register1: T, register2: T) -> T {
//     todo!("Eviction policy not implemented");
// }

pub fn opt_miss_ratio<T: PartialEq + Eq + Clone + Hash>(trace: &[T], cache_size: usize) -> f64 {
    let mut cache_accesses: usize = 0;
    let mut cache_misses: usize = 0;
    let mut cache_registers: Vec<T> = Vec::with_capacity(cache_size);
    let mut cache_distances: Vec<usize> = Vec::with_capacity(cache_size);
    let forward_distances = forward_distance(trace);

    for (time, element) in trace.iter().enumerate() {
        // Access cache
        cache_accesses += 1;

        if !cache_registers.contains(element) {
            cache_misses += 1;
            if cache_registers.len() == cache_size {
                // Evict
                let evict_index = cache_distances
                    .iter()
                    .position(|x| x == cache_distances.iter().max().unwrap())
                    .unwrap();
                cache_registers[evict_index] = element.clone();
                cache_distances[evict_index] = forward_distances[time];
            } else {
                cache_registers.push(element.clone());
                cache_distances.push(forward_distances[time]);
            }
        }

        // Update cache
        for i in cache_distances.iter_mut() {
            if *i == 0 {
                *i = forward_distances[time];
            }
            *i -= 1;
        }
    }

    cache_misses as f64 / cache_accesses as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_distance_test() {
        let trace = vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 5, 6, 5, 6];
        let result = forward_distance(&trace);
        assert_eq!(
            result,
            [
                4,
                4,
                4,
                4,
                4,
                4,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                2,
                2,
                usize::MAX,
                usize::MAX
            ]
        );
    }

    #[test]
    fn opt_miss_ratio_test() {
        let trace = vec![1, 213, 43, 1, 2, 3, 4, 213, 9, 5, 43, 1, 5, 32, 213];
        // let result = opt_miss_ratio(&trace, 4);
        // assert_eq!(result, 1.0 / 3.0);
        // let result = opt_miss_ratio(&trace, 2);
        // assert_eq!(result, 1.0 / 3.0);
        let result = opt_miss_ratio(&trace, 100);
        assert_eq!(result, 0.6);
    }
}
