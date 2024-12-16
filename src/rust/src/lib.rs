use savvy::{savvy, NumericSexp, NumericTypedSexp, IntegerSexp, RealSexp, Sexp};
use std::thread;

/// Calculate the sum of a vector of integers/doubles using multiple threads.
///
/// @param x A vector of integers/doubles to sum over.
/// @param n The number of threads used to compute this calculation (int).
///
/// @return The sum of all elements of the input vector.
///
/// @export
#[savvy]
fn sum_with_threads(x: NumericSexp, n: i32) -> savvy::Result<savvy::Sexp> {
    match x.into_typed() {
        NumericTypedSexp::Integer(i) => sum_with_threads_int(i, n),
        NumericTypedSexp::Real(r) => sum_with_threads_rel(r, n),
    }
}

fn sum_with_threads_int(x: IntegerSexp, n: i32) -> savvy::Result<Sexp> {
    let x_rust = x.to_vec();
    let n_usize: usize = n as usize;

    let out = sum_with_threads_int_impl(x_rust, n_usize);
    out.try_into()
}

fn sum_with_threads_rel(x: RealSexp, n: i32) -> savvy::Result<Sexp> {
    let x_rust = x.to_vec();
    let n_usize: usize = n as usize;

    let out = sum_with_threads_rel_impl(x_rust, n_usize);
    out.try_into()
}

fn sum_with_threads_int_impl(x: Vec<i32>, n: usize) -> i32 {
    if x.is_empty() {
        eprintln!("Input vector is empty. Returning 0.");
        return 0;
    }

    let n = n.min(x.len());
    let chunk_size = (x.len() + n - 1) / n;

    let mut handles = Vec::new();
    for i in 0..n {
        let chunk = x[i * chunk_size..((i + 1) * chunk_size).min(x.len())].to_vec();
        handles.push(thread::spawn(move || chunk.iter().sum::<i32>()));
    }

    let mut total_sum = 0;
    for handle in handles {
        total_sum += handle.join().expect("Thread panicked");
    }

    total_sum
}

fn sum_with_threads_rel_impl(x: Vec<f64>, n: usize) -> f64 {
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
    use crate::{sum_with_threads_int_impl, sum_with_threads_rel_impl};

 /// Integers
    #[test]
    fn test_single_thread_int() {
        let numbers = vec![1, 2, 3, 4, 5];
        let n = 1;
        assert_eq!(sum_with_threads_int_impl(numbers, n), 15);
    }

    #[test]
    fn test_multiple_threads_int() {
        let x = vec![1, 2, 3, 4];
        let num_threads = 2;

        let result = sum_with_threads_int_impl(x, num_threads);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_more_threads_than_elements_int() {
        let numbers = vec![1, 2, 3, 4, 5];
        let n = 10;
        assert_eq!(sum_with_threads_int_impl(numbers, n), 15);
    }

    #[test]
    fn test_empty_vector_int() {
        let numbers: Vec<i32> = vec![];
        let n = 4;
        assert_eq!(sum_with_threads_int_impl(numbers, n), 0);
    }

    #[test]
    fn test_large_numbers_int() {
        let numbers = vec![1_000_000, 2_000_000, 3_000_000];
        let n = 3;
        assert_eq!(sum_with_threads_int_impl(numbers, n), 6_000_000);
    }

    #[test]
    fn test_negative_numbers_int() {
        let numbers = vec![-1, -2, -3, -4, -5];
        let n = 2;
        assert_eq!(sum_with_threads_int_impl(numbers, n), -15);
    }

    #[test]
    fn test_mixed_numbers_int() {
        let numbers = vec![-1, 2, -3, 4, -5, 6];
        let n = 3;
        assert_eq!(sum_with_threads_int_impl(numbers, n), 3);
    }

    #[test]
    fn test_large_vector_int() {
        let numbers: Vec<i32> = (1..=1_000).collect();
        let n = 4;
        let expected_sum: i32 = (1..=1_000).sum();
        assert_eq!(sum_with_threads_int_impl(numbers, n), expected_sum);
    }
}

/// Real/doubles
#[test]
fn test_single_thread_rel() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let n = 1;
    assert_eq!(sum_with_threads_rel_impl(numbers, n), 15.0);
}

#[test]
fn test_multiple_threads_rel() {
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let num_threads = 2;

    let result = sum_with_threads_rel_impl(x, num_threads);
    assert_eq!(result, 10.0);
}

#[test]
fn test_more_threads_than_elements_rel() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let n = 10;
    assert_eq!(sum_with_threads_rel_impl(numbers, n), 15.0);
}

#[test]
fn test_empty_vector_rel() {
    let numbers: Vec<f64> = vec![];
    let n = 4;
    assert_eq!(sum_with_threads_rel_impl(numbers, n), 0);
}

#[test]
fn test_large_numbers_rel() {
    let numbers = vec![1_000_000, 2_000_000, 3_000_000];
    let n = 3;
    assert_eq!(sum_with_threads_rel_impl(numbers, n), 6_000_000);
}

#[test]
fn test_negative_numbers_rel() {
    let numbers = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
    let n = 2;
    assert_eq!(sum_with_threads_rel_impl(numbers, n), -15.0);
}

#[test]
fn test_mixed_numbers_rel() {
    let numbers = vec![-1.0, 2.0, -3.0, 4.0, -5.0, 6.0];
    let n = 3;
    assert_eq!(sum_with_threads_rel_impl(numbers, n), 3.0);
}

#[test]
fn test_large_vector_rel() {
    let numbers: Vec<f64> = (1..=1_000).collect();
    let n = 4;
    let expected_sum: i32 = (1..=1_000).sum();
    assert_eq!(sum_with_threads_rel_impl(numbers, n), expected_sum);
}
