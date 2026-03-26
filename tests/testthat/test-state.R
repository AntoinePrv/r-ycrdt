for (version in c("v1", "v2")) {
  local(
    {
      test_that(
        paste("StateVector decode", version, "errors on invalid data"),
        {
          expect_s3_class(
            StateVector[[paste0("decode_", version)]](as.raw(c(0xff))),
            "extendr_error"
          )
        }
      )
    },
    list(version = version)
  )
}
