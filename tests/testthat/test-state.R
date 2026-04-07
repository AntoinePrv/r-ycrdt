for (version in c("v1", "v2")) {
  local(
    {
      encode <- paste0("encode_", version)
      decode <- paste0("decode_", version)

      test_that(
        paste("StateVector encode/decode", version, "roundtrip"),
        {
          doc <- Doc$new()
          text <- doc$get_or_insert_text("t")
          doc$with_transaction(
            function(trans) text$push(trans, "hello"),
            mutable = TRUE
          )
          sv <- doc$with_transaction(function(trans) trans$state_vector())
          encoded <- sv[[encode]]()
          expect_type(encoded, "raw")
          decoded <- StateVector[[decode]](encoded)
          expect_false(decoded$is_empty())
          expect_equal(decoded$len(), sv$len())
        }
      )

      test_that(
        paste("StateVector decode", version, "errors on invalid data"),
        {
          expect_s3_class(
            StateVector[[decode]](as.raw(c(0xff))),
            "extendr_error"
          )
        }
      )
    },
    list(version = version)
  )
}

test_that("StateVector equality and ordering", {
  doc <- Doc$new()
  text <- doc$get_or_insert_text("t")

  sv1 <- doc$with_transaction(function(txn) txn$state_vector())
  sv2 <- doc$with_transaction(function(txn) txn$state_vector())
  expect_true(sv1 == sv2)
  expect_false(sv1 != sv2)
  expect_true(sv1 <= sv1)
  expect_true(sv1 >= sv1)

  doc$with_transaction(function(txn) text$push(txn, "x"), mutable = TRUE)
  sv3 <- doc$with_transaction(function(txn) txn$state_vector())
  expect_false(sv1 == sv3)
  expect_true(sv1 != sv3)
  expect_true(sv1 < sv3)
  expect_true(sv1 <= sv3)
  expect_false(sv1 > sv3)
  expect_false(sv1 >= sv3)
  expect_true(sv3 > sv1)
  expect_true(sv3 >= sv1)
})

#############
# DeleteSet #
#############

test_that("DeleteSet$new() is empty", {
  ds <- DeleteSet$new()
  expect_true(ds$is_empty())
  expect_equal(ds$len(), 0L)
})

for (version in c("v1", "v2")) {
  local(
    {
      encode <- paste0("encode_", version)
      decode <- paste0("decode_", version)

      test_that(
        paste("DeleteSet encode/decode", version, "roundtrip"),
        {
          ds <- DeleteSet$new()
          encoded <- ds[[encode]]()
          expect_type(encoded, "raw")
          decoded <- DeleteSet[[decode]](encoded)
          expect_true(decoded$is_empty())
          expect_equal(decoded$len(), 0L)
        }
      )

      test_that(
        paste("DeleteSet decode", version, "errors on invalid data"),
        {
          expect_s3_class(
            DeleteSet[[decode]](as.raw(c(0xff))),
            "extendr_error"
          )
        }
      )
    },
    list(version = version)
  )
}

test_that("DeleteSet is_deleted returns FALSE for empty set", {
  ds <- DeleteSet$new()
  expect_false(ds$is_deleted(list(client = 1L, clock = 0L)))
})

# v1 encoding: 1 client (id=1), 1 range [clock=0, len=3)
# Bytes: num_clients=1, client_id=1, num_ranges=1, clock=0, len=3
ds_v1_bytes <- as.raw(c(0x01, 0x01, 0x01, 0x00, 0x03))

test_that("DeleteSet is_deleted returns TRUE for deleted ID", {
  ds <- DeleteSet$decode_v1(ds_v1_bytes)
  expect_false(ds$is_empty())
  expect_equal(ds$len(), 1L)
  expect_true(ds$is_deleted(list(client = 1L, clock = 0L)))
  expect_true(ds$is_deleted(list(client = 1L, clock = 2L)))
})

test_that("DeleteSet is_deleted returns FALSE for non-deleted ID", {
  ds <- DeleteSet$decode_v1(ds_v1_bytes)
  expect_false(ds$is_deleted(list(client = 1L, clock = 3L)))
  expect_false(ds$is_deleted(list(client = 2L, clock = 0L)))
})

test_that("DeleteSet equality", {
  ds1 <- DeleteSet$new()
  ds2 <- DeleteSet$new()
  expect_true(ds1 == ds2)
  expect_false(ds1 != ds2)

  ds3 <- DeleteSet$decode_v1(ds_v1_bytes)
  expect_false(ds1 == ds3)
  expect_true(ds1 != ds3)

  ds4 <- DeleteSet$decode_v1(ds_v1_bytes)
  expect_true(ds3 == ds4)
  expect_false(ds3 != ds4)
})

############
# Snapshot #
############

test_that("Snapshot$new() from empty StateVector and DeleteSet", {
  sv <- StateVector$decode_v1(as.raw(0x00))
  ds <- DeleteSet$new()
  snap <- Snapshot$new(sv, ds)
  encoded <- snap$encode_v1()
  expect_type(encoded, "raw")
})

for (version in c("v1", "v2")) {
  local(
    {
      encode <- paste0("encode_", version)
      decode <- paste0("decode_", version)

      test_that(
        paste("Snapshot encode/decode", version, "roundtrip"),
        {
          sv <- StateVector$decode_v1(as.raw(0x00))
          ds <- DeleteSet$new()
          snap <- Snapshot$new(sv, ds)
          encoded <- snap[[encode]]()
          expect_type(encoded, "raw")
          decoded <- Snapshot[[decode]](encoded)
          expect_true(decoded == snap)
        }
      )

      test_that(
        paste("Snapshot decode", version, "errors on invalid data"),
        {
          expect_s3_class(
            Snapshot[[decode]](as.raw(c(0xff))),
            "extendr_error"
          )
        }
      )
    },
    list(version = version)
  )
}

test_that("Snapshot equality", {
  sv1 <- StateVector$decode_v1(as.raw(0x00))
  sv2 <- StateVector$decode_v1(as.raw(0x00))
  ds <- DeleteSet$new()
  snap1 <- Snapshot$new(sv1, ds)
  snap2 <- Snapshot$new(sv2, ds)
  expect_true(snap1 == snap2)
  expect_false(snap1 != snap2)
})
