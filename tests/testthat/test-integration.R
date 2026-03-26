# This is the quick start example from yrs, https://docs.rs/yrs/latest/yrs/
for (version in c("v1", "v2")) {
  local(
    {
      test_that(paste("Synchronize two docs", version), {
        doc <- yar::Doc$new()
        text <- doc$get_or_insert_text("article")

        doc$with_transaction(
          function(trans) {
            text$insert(trans, 0L, "hello")
            text$insert(trans, 5L, " world")
            trans$commit()

            expect_equal(text$get_string(trans), "hello world")
          },
          mutable = TRUE
        )

        # Synchronize state with remote replica
        remote_doc <- yar::Doc$new()
        remote_text <- remote_doc$get_or_insert_text("article")

        remote_sv_raw <- remote_doc$with_transaction(function(remote_trans) {
          remote_trans$state_vector()[[paste0("encode_", version)]]()
        })

        # Get update with contents not observed by remote_doc
        update <- doc$with_transaction(function(local_trans) {
          remote_sv <- yar::StateVector[[paste0("decode_", version)]](
            remote_sv_raw
          )
          local_trans[[paste0("encode_diff_", version)]](remote_sv)
        })

        # Apply update on remote doc
        remote_doc$with_transaction(
          function(remote_trans) {
            remote_trans[[paste0("apply_update_", version)]](update)
            remote_trans$commit()

            expect_equal(remote_text$get_string(remote_trans), "hello world")
          },
          mutable = TRUE
        )
      })
    },
    list(version = version)
  )
}
