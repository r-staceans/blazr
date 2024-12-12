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
mod test1 {
    use crate::sum_with_threads_impl;

    #[test]
    fn test_sum_with_threads_impl_basic() {
        let x = vec![1, 2, 3, 4];
        let num_threads = 2;

        let result = sum_with_threads_impl(x, num_threads);
        assert_eq!(result, 10);
    }
}
