pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Implementing with basic simulation
pub fn opt_miss_ratio_simulation<T : PartialEq + Eq + Clone>(trace: &[T], cache_size: usize) -> f64 {
    todo!()
}

// Implementing with forward distance algorithm
pub fn opt_miss_ratio_forward_distance<T : PartialEq + Eq + Clone>(trace: &[T], cache_size: usize) -> f64 {
    todo!()
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
