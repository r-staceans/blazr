use savvy::{savvy, RealSexp};
use std::thread;

/// Calculate the sum of a vector of real numbers using multiple threads.
///
/// @param x A vector of real numbers to sum over.
/// @param n The number of threads used to compute this calculation (int).
///
/// @return The sum of all elements of the input vector.
#[savvy]
fn sum_with_threads_real(x: RealSexp, n: i32) -> savvy::Result<savvy::Sexp> {
    let x_rust = x.to_vec();
    let n_size: usize = n as usize;

    let out = sum_with_threads_real_impl(x_rust, n_size);
    out.try_into()
}

fn sum_with_threads_real_impl(x: Vec<f64>, n: usize) -> f64 {
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
    use crate::sum_with_threads_real_impl;

    /// Real/doubles
    #[test]
    fn test_single_thread_rel() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let n = 1;
        assert_eq!(sum_with_threads_real_impl(numbers, n), 15.0);
    }

    #[test]
    fn test_multiple_threads_rel() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let num_threads = 2;

        let result = sum_with_threads_real_impl(x, num_threads);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_more_threads_than_elements_rel() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let n = 10;
        assert_eq!(sum_with_threads_real_impl(numbers, n), 15.0);
    }

    #[test]
    fn test_empty_vector_rel() {
        let numbers: Vec<f64> = vec![];
        let n = 4;
        assert_eq!(sum_with_threads_real_impl(numbers, n), 0.0);
    }

    #[test]
    fn test_large_numbers_rel() {
        let numbers = vec![1_000_000.0, 2_000_000.0, 3_000_000.0];
        let n = 3;
        assert_eq!(sum_with_threads_real_impl(numbers, n), 6_000_000.0);
    }

    #[test]
    fn test_negative_numbers_rel() {
        let numbers = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
        let n = 2;
        assert_eq!(sum_with_threads_real_impl(numbers, n), -15.0);
    }

    #[test]
    fn test_mixed_numbers_rel() {
        let numbers = vec![-1.0, 2.0, -3.0, 4.0, -5.0, 6.0];
        let n = 3;
        assert_eq!(sum_with_threads_real_impl(numbers, n), 3.0);
    }

    #[test]
    fn test_large_vector_rel() {
        let numbers: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
        let n = 4;
        let expected_sum: f64 = numbers.iter().sum();
        assert_eq!(sum_with_threads_real_impl(numbers, n), expected_sum);
    }
}
