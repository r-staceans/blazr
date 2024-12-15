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
