pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn opt_miss_ratio<T : PartialEq + Eq>(trace: &[T], cache_size: usize) -> f64 {
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
