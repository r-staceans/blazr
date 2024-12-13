test_that("sums as expected", {
  vector <- 1:10

  out <- blazr::sum_with_threads(1:10, 5L)
  expect_equal(out, sum(vector))
})
