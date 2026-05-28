#' @include extendr-wrappers.R error-wrapper.R
NULL

#' @export Update
NULL

#' @export
`$.Update` <- make_dispatcher(Update)

#' @export
`[[.Update` <- `$.Update`

install_checked_static(Update, "decode_v1")
install_checked_static(Update, "decode_v2")

#' @export
print.Update <- function(x, ...) {
  cat(x$to_string(), "\n", sep = "")
  invisible(x)
}
