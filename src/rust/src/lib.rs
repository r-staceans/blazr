// This test is run by `cargo test`. You can put tests that don't need a real
// R session here.
#[cfg(test)]
mod test1 {
    use crate::sum_with_threads_impl;

    #[test]
    fn test_sum_with_threads_impl_basic {
        let x = vec![1, 2, 3, 4];
        let num_threads = 2;

        let result = sum_with_threads_impl(x, num_threads);
        assert_eq!(result, 10);
    }
}
