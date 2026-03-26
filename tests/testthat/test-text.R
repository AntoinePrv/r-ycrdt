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
