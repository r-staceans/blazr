use savvy::{savvy, ComplexSexp, Sexp};
use std::thread;

/// Calculate the sum of a vector of integers using multiple threads.
///
/// @param x A vector of integers to sum over.
/// @param n The number of threads used to compute this calculation (int).
///
/// @return The sum of all elements of the input vector.
///
/// @export
#[savvy]
fn sum_with_threads(x: ComplexSexp, n: i32) -> savvy::Result<Sexp> {
    let x_rust = x.to_vec();
    let n_usize: usize = n as usize;

    let out = sum_with_threads_impl(x_rust, n_usize);
    out.try_into()
}

fn sum_with_threads_impl(x: Vec<num_complex::Complex<f64>>, n: usize) -> f64 {
    if x.is_empty() {
        eprintln!("Input vector is empty. Returning 0.");
        return 0.0;
    }

    let n = n.min(x.len());
    let chunk_size = (x.len() + n - 1) / n;

    let mut handles = Vec::new();
    for i in 0..n {
        let chunk = x[i * chunk_size..((i + 1) * chunk_size).min(x.len())].to_vec();
        handles.push(thread::spawn(move || chunk.iter().sum::<f64>()));
    }

    let mut total_sum = 0.0;
    for handle in handles {
        total_sum += handle.join().expect("Thread panicked");
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use crate::sum_with_threads_impl;

    #[test]
    fn test_single_thread() {
        let numbers = vec![1, 2, 3, 4, 5];
        let n = 1;
        assert_eq!(sum_with_threads_impl(numbers, n), 15);
    }

    #[test]
    fn test_multiple_threads() {
        let x = vec![1, 2, 3, 4];
        let num_threads = 2;

        let result = sum_with_threads_impl(x, num_threads);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_more_threads_than_elements() {
        let numbers = vec![1, 2, 3, 4, 5];
        let n = 10;
        assert_eq!(sum_with_threads_impl(numbers, n), 15);
    }

    #[test]
    fn test_empty_vector() {
        let numbers: Vec<i32> = vec![];
        let n = 4;
        assert_eq!(sum_with_threads_impl(numbers, n), 0);
    }

    #[test]
    fn test_large_numbers() {
        let numbers = vec![1_000_000, 2_000_000, 3_000_000];
        let n = 3;
        assert_eq!(sum_with_threads_impl(numbers, n), 6_000_000);
    }

    #[test]
    fn test_negative_numbers() {
        let numbers = vec![-1, -2, -3, -4, -5];
        let n = 2;
        assert_eq!(sum_with_threads_impl(numbers, n), -15);
    }

    #[test]
    fn test_mixed_numbers() {
        let numbers = vec![-1, 2, -3, 4, -5, 6];
        let n = 3;
        assert_eq!(sum_with_threads_impl(numbers, n), 3);
    }

    #[test]
    fn test_doubles_numbers() {
        let numbers = vec![-1.5, 2.0, -3.5, 4.0, -5.0, 6.5];
        let n = 3;
        assert_eq!(sum_with_threads_impl(numbers, n), 2.5);
    }

    #[test]
    fn test_large_vector() {
        let numbers: Vec<i32> = (1..=1_000).collect();
        let n = 4;
        let expected_sum: i32 = (1..=1_000).sum();
        assert_eq!(sum_with_threads_impl(numbers, n), expected_sum);
    }
}
