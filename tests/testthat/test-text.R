test_that("Text insert and retrieve get_string", {
  doc <- Doc$new()
  text <- doc$get_or_insert_text("article")

  doc$with_transaction(
    function(trans) {
      text$insert(trans, 0L, "hello")
      text$insert(trans, 5L, " world")
      trans$commit()

      expect_equal(text$get_string(trans), "hello world")
      expect_equal(text$len(trans), 11L)
    },
    mutable = TRUE
  )
})

test_that("Text push appends to the end", {
  doc <- Doc$new()
  text <- doc$get_or_insert_text("article")

  doc$with_transaction(
    function(trans) {
      text$push(trans, "hello")
      text$push(trans, " world")

      expect_equal(text$get_string(trans), "hello world")
    },
    mutable = TRUE
  )
})

test_that("Text remove_range removes characters", {
  doc <- Doc$new()
  text <- doc$get_or_insert_text("article")

  doc$with_transaction(
    function(trans) {
      text$push(trans, "hello world")
      text$remove_range(trans, 5L, 6L)

      expect_equal(text$get_string(trans), "hello")
    },
    mutable = TRUE
  )
})

####################
# Observer pattern #
####################

test_that("Text observe callback can read current state via transaction", {
  doc <- Doc$new()
  text <- doc$get_or_insert_text("article")

  called <- FALSE
  observed <- NULL
  text$observe(
    function(trans, event) {
      called <<- TRUE
      observed <<- text$get_string(trans)
    },
    key = 1L
  )

  doc$with_transaction(
    function(trans) text$push(trans, "hello"),
    mutable = TRUE
  )

  expect_true(called)
  expect_equal(observed, "hello")
})

test_that("Text observe callback transaction cannot be used after callback returns", {
  doc <- Doc$new()
  text <- doc$get_or_insert_text("article")

  captured <- NULL
  text$observe(
    function(trans, event) {
      captured <<- trans
    },
    key = 1L
  )

  doc$with_transaction(
    function(trans) text$push(trans, "hello"),
    mutable = TRUE
  )

  expect_s3_class(
    text$get_string(captured),
    "extendr_error"
  )
})
