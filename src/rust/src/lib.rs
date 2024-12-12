use std::thread;

fn sum_with_threads_impl(x: Vec<i32>, n: usize) -> i32 {
    // Ensure n doesn't exceed the number of elements
    let n = n.min(x.len());
    let chunk_size = (x.len() + n - 1) / n; // Divide the elements into approximately equal chunks

    // Create threads to compute the sum of each chunk
    let mut handles = Vec::new();
    for i in 0..n {
        let chunk = x[i * chunk_size..((i + 1) * chunk_size).min(x.len())].to_vec();
        handles.push(thread::spawn(move || chunk.iter().sum::<i32>()));
    }

    // Collect the partial results from each thread
    let mut total_sum = 0;
    for handle in handles {
        total_sum += handle.join().expect("Thread panicked");
    }

    total_sum
}

// This test is run by `cargo test`. You can put tests that don't need a real
// R session here.
#[cfg(test)]
mod tests {
    use crate::sum_with_threads_impl;

    #[test]
    fn test_single_thread() {
        let numbers = vec![1, 2, 3, 4, 5];
        let n = 1; // Single thread
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
        let n = 10; // More threads than elements
        assert_eq!(sum_with_threads_impl(numbers, n), 15);
    }

    #[test]
    fn test_empty_vector() {
        let numbers: Vec<i32> = vec![];
        let n = 4; // Any number of threads
        assert_eq!(sum_with_threads_impl(numbers, n), 0);
    }

    #[test]
    fn test_large_numbers() {
        let numbers = vec![1_000_000, 2_000_000, 3_000_000];
        let n = 3; // Three threads
        assert_eq!(sum_with_threads_impl(numbers, n), 6_000_000);
    }

    #[test]
    fn test_negative_numbers() {
        let numbers = vec![-1, -2, -3, -4, -5];
        let n = 2; // Two threads
        assert_eq!(sum_with_threads_impl(numbers, n), -15);
    }

    #[test]
    fn test_mixed_numbers() {
        let numbers = vec![-1, 2, -3, 4, -5, 6];
        let n = 3; // Three threads
        assert_eq!(sum_with_threads_impl(numbers, n), 3);
    }

    #[test]
    fn test_large_vector() {
        let numbers: Vec<i32> = (1..=1_000).collect(); // Vector with numbers from 1 to 1000
        let n = 4; // Four threads
        let expected_sum: i32 = (1..=1_000).sum();
        assert_eq!(sum_with_threads_impl(numbers, n), expected_sum);
    }
}
