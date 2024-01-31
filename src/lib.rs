pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn forward_distance<T : PartialEq + Eq + Clone> (trace: &[T]) -> Vec<usize>{
    let mut registers: Vec<T> = Vec::with_capacity(trace.len());
    let mut counter: Vec<usize> = Vec::with_capacity(trace.len());
    let mut distances: Vec<usize> = Vec::with_capacity(trace.len());
    let mut index: usize = 0;

    for i in trace.len()-1..=0 {
        let element = &trace[i];

        // Update counter
        for j in 0..registers.len() {
            counter[j] += 1;
        }

        // Update registers and distances
        if !registers.contains(element) {
            registers.push(element.clone());
            counter.push(usize::MAX);
        } else {
            index = registers.iter().position(|x| *x == *element).unwrap();
            distances.insert(index, counter[index]);
            counter[index] = 0;
        }
    }

    return distances;
}

pub fn evicted_by_rule_omega<T : PartialEq + Eq + Clone>(register1: T, register2: T) -> T {
    todo!("Eviction policy not implemented");
}

pub fn opt_miss_ratio<T : PartialEq + Eq + Clone>(trace: &[T], cache_size: usize) -> f64 {
    let mut cache_accesses: usize = 0;
    let mut cache_misses: usize = 0;
    let mut cache_registers: Vec<T> = Vec::with_capacity(cache_size);
    let mut cache_distances: Vec<usize> = Vec::with_capacity(cache_size);
    let mut time: usize = 0;
    let mut evict_index: usize = 0;

    let forward_distances = forward_distance(trace);
    
    for element in trace {
        // Access cache
        cache_accesses += 1;
        if !cache_registers.contains(element) {
            cache_misses += 1;
            if cache_registers.len() == cache_size {
                // Evict
                evict_index = cache_distances.iter().position(|x| x == cache_distances.iter().max().unwrap()).unwrap();
                cache_registers.remove(evict_index);
                cache_distances.remove(evict_index);
            }
            cache_registers.push(element.clone());
            cache_distances.push(forward_distances[time]);
        }

        // Update cache
        for i in 0..cache_registers.len() {
            cache_distances[i] -= 1;
        }

        // Update time
        time += 1;
    }
    
    return cache_misses as f64 / cache_accesses as f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
