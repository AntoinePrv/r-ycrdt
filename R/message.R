#' @include extendr-wrappers.R
NULL

#' @export SyncMessage
NULL

#' @export Message
NULL

#' @export
`==.SyncMessage` <- function(e1, e2) {
  e1$equal(e2)
}

#' @export
`!=.SyncMessage` <- function(e1, e2) {
  e1$not_equal(e2)
}
