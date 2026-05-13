test_that("Map insert methods return usable nested types", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  doc$with_transaction(
    function(trans) {
      map$insert_any(trans, "string", "hello")
      map$insert_any(trans, "number", 1.5)
      map$insert_any(trans, "integer", 42L)
      map$insert_any(trans, "bool", TRUE)
      expect_equal(map$get(trans, "string"), "hello")
      expect_equal(map$get(trans, "number"), 1.5)
      expect_equal(map$get(trans, "integer"), 42L)
      expect_equal(map$get(trans, "bool"), TRUE)

      text <- map$insert_text(trans, "content")
      expect_true(inherits(text, "TextRef"))
      expect_true(inherits(map$get(trans, "content"), "TextRef"))
      text$push(trans, "hello")
      text$push(trans, " world")
      expect_equal(text$get_string(trans), "hello world")

      arr <- map$insert_array(trans, "list")
      expect_true(inherits(arr, "ArrayRef"))
      expect_true(inherits(map$get(trans, "list"), "ArrayRef"))
      arr$insert_any(trans, 0L, TRUE)
      expect_equal(arr$len(trans), 1L)

      nested <- map$insert_map(trans, "nested")
      expect_true(inherits(nested, "MapRef"))
      expect_true(inherits(map$get(trans, "nested"), "MapRef"))
      nested$insert_any(trans, "k", 42L)
      expect_equal(nested$len(trans), 1L)

      expect_equal(map$len(trans), 7L)
      expect_null(map$get(trans, "missing"))
    },
    mutable = TRUE
  )
})

test_that("Map keys returns all keys", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  doc$with_transaction(
    function(trans) {
      map$insert_any(trans, "a", 1L)
      map$insert_any(trans, "b", 2L)

      expect_setequal(map$keys(trans), c("a", "b"))
    },
    mutable = TRUE
  )
})

test_that("Map items returns named list of values", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  doc$with_transaction(
    function(trans) {
      map$insert_any(trans, "x", 1.5)
      map$insert_any(trans, "y", "hello")

      result <- map$items(trans)
      expect_true(is.list(result))
      expect_setequal(names(result), c("x", "y"))
      expect_equal(result[["x"]], 1.5)
      expect_equal(result[["y"]], "hello")
    },
    mutable = TRUE
  )
})

test_that("Map items returns nested types", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  doc$with_transaction(
    function(trans) {
      map$insert_text(trans, "t")
      map$insert_array(trans, "a")
      map$insert_map(trans, "m")

      result <- map$items(trans)
      expect_true(inherits(result[["t"]], "TextRef"))
      expect_true(inherits(result[["a"]], "ArrayRef"))
      expect_true(inherits(result[["m"]], "MapRef"))
    },
    mutable = TRUE
  )
})

test_that("Map insert_text and contains_key", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  doc$with_transaction(
    function(trans) {
      map$insert_text(trans, "key")

      expect_equal(map$len(trans), 1L)
      expect_true(map$contains_key(trans, "key"))
      expect_false(map$contains_key(trans, "other"))
    },
    mutable = TRUE
  )
})

test_that("Map remove decreases len", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  doc$with_transaction(
    function(trans) {
      map$insert(trans, "a", Prelim$any(1L))
      map$insert(trans, "b", Prelim$any(2L))
      map$remove(trans, "a")

      expect_equal(map$len(trans), 1L)
      expect_false(map$contains_key(trans, "a"))
    },
    mutable = TRUE
  )
})

test_that("Map insert with Prelim variants stores usable values", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  doc$with_transaction(
    function(trans) {
      map$insert(trans, "string", Prelim$any("hello"))
      map$insert(trans, "integer", Prelim$any(42L))
      map$insert(trans, "text", Prelim$text("hi"))
      map$insert(trans, "array", Prelim$array(list("a", "b")))
      map$insert(trans, "map", Prelim$map(list(a = 1L, b = 2L)))
      # detect dispatches based on input shape
      map$insert(trans, "detected", Prelim$detect(list(x = TRUE)))

      expect_equal(map$len(trans), 6L)
      expect_equal(map$get(trans, "string"), "hello")
      expect_equal(map$get(trans, "integer"), 42L)

      text <- map$get(trans, "text")
      expect_true(inherits(text, "TextRef"))
      text$push(trans, "!")
      expect_equal(text$get_string(trans), "hi!")

      arr <- map$get(trans, "array")
      expect_true(inherits(arr, "ArrayRef"))
      arr$insert(trans, 2L, Prelim$any("c"))
      expect_equal(arr$len(trans), 3L)
      expect_equal(arr$get(trans, 2L), "c")

      nested_map <- map$get(trans, "map")
      expect_true(inherits(nested_map, "MapRef"))
      expect_equal(nested_map$len(trans), 2L)
      expect_equal(nested_map$get(trans, "a"), 1L)

      detected_map <- map$get(trans, "detected")
      expect_true(inherits(detected_map, "MapRef"))
      expect_equal(detected_map$get(trans, "x"), TRUE)
    },
    mutable = TRUE
  )
})

test_that("Map clear removes all entries", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  doc$with_transaction(
    function(trans) {
      map$insert_text(trans, "a")
      map$insert_text(trans, "b")
      map$clear(trans)

      expect_equal(map$len(trans), 0L)
    },
    mutable = TRUE
  )
})

####################
# Observer pattern #
####################

test_that("Map observe callback can read current state via transaction", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  called <- FALSE
  observed_len <- NULL
  observed_target_len <- NULL
  observed_path <- NULL
  observed_keys <- NULL
  map$observe(
    function(trans, event) {
      called <<- TRUE
      observed_len <<- map$len(trans)
      observed_target_len <<- event$target()$len(trans)
      observed_path <<- event$path()
      observed_keys <<- event$keys(trans)
    },
    key = 1L
  )

  doc$with_transaction(
    function(trans) map$insert_any(trans, "foo", 42L),
    mutable = TRUE
  )

  expect_true(called)
  expect_equal(observed_len, 1L)
  expect_equal(observed_target_len, 1L)
  expect_equal(observed_path, list())
  expect_true(is.list(observed_keys))
  expect_true("foo" %in% names(observed_keys))
})

test_that("Map unobserve stops callback from firing", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  count <- 0L
  map$observe(
    function(trans, event) count <<- count + 1L,
    key = 1L
  )

  doc$with_transaction(
    function(trans) map$insert_any(trans, "a", 1L),
    mutable = TRUE
  )
  expect_equal(count, 1L)

  map$unobserve(key = 1L)

  doc$with_transaction(
    function(trans) map$insert_any(trans, "b", 2L),
    mutable = TRUE
  )
  expect_equal(count, 1L)
})

test_that("Map observe callback transaction cannot be used after callback returns", {
  doc <- Doc$new()
  map <- doc$get_or_insert_map("data")

  captured_trans <- NULL
  captured_event <- NULL
  map$observe(
    function(trans, event) {
      captured_trans <<- trans
      captured_event <<- event
    },
    key = 1L
  )

  doc$with_transaction(
    function(trans) map$insert_any(trans, "foo", 1L),
    mutable = TRUE
  )

  # Captured objects are invalidated
  expect_s3_class(
    map$len(captured_trans),
    "extendr_error"
  )
  expect_s3_class(
    captured_event$path(),
    "extendr_error"
  )
})
