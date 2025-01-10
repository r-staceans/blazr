#' Calculate the sum of a vector of numbers using multiple threads.
#'
#' @param x A vector of numbers to sum over.
#' @param n The number of threads used to compute this calculation (int).
#'
#' @return The sum of all elements of the input vector.
#' @export
#'
#' @examples
#' sum_with_threads(c(1,2), 2)
sum_with_threads <- function(x, n) {

  tryCatch(
    x <- vctrs::vec_cast(x, double()),
    error = function(e) {
      stop("x must be coercible to a numeric vector")
    }
  )

  tryCatch(
    n <- vctrs::vec_cast(n, integer()),
    error = function(e) {
      stop("n must be coercible to an integer")
    }
  )

  sum_with_threads_real(x, n)
}
