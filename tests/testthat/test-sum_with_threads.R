test_that("sums as integers expected", {
  vector_int <- 1:10

  out <- blazr::sum_with_threads(1:10, 5L)
  expect_equal(out, sum(vector_int))
})

test_that("sums as doubles expected", {
  vector_dbl <- c(1.5, 2.7, 4.25)

  out <- blazr::sum_with_threads(vector_dbl, 5L)
  expect_equal(out, sum(vector_dbl))
})

test_that("with non-numeric vectors, errors as expected", {
  vector_chr <- c("a", "b", "c")

  expect_error(
    blazr::sum_with_threads(vector_chr, 5L),
    "must be coercible to a numeric vector"
    )
})

test_that("with non-integer threads, errors as expected", {
  vector_int <- 1:10

  expect_error(
    blazr::sum_with_threads(1:10, 5.5),
    "must be coercible to an integer"
    )
})
