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
