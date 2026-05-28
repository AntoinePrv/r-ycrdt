#' @include extendr-wrappers.R error-wrapper.R
NULL

#' @export StateVector
NULL

#' @export
`$.StateVector` <- make_dispatcher(StateVector)

#' @export
`[[.StateVector` <- `$.StateVector`

#' @export
`$.DeleteSet` <- make_dispatcher(DeleteSet)

#' @export
`[[.DeleteSet` <- `$.DeleteSet`

#' @export
`$.Snapshot` <- make_dispatcher(Snapshot)

#' @export
`[[.Snapshot` <- `$.Snapshot`

install_checked_static(StateVector, "decode_v1")
install_checked_static(StateVector, "decode_v2")
install_checked_static(DeleteSet, "decode_v1")
install_checked_static(DeleteSet, "decode_v2")
install_checked_static(Snapshot, "decode_v1")
install_checked_static(Snapshot, "decode_v2")

#' @export
`==.StateVector` <- function(e1, e2) {
  e1$equal(e2)
}

#' @export
`!=.StateVector` <- function(e1, e2) {
  e1$not_equal(e2)
}

#' @export
`<.StateVector` <- function(e1, e2) {
  e1$less_than(e2)
}

#' @export
`<=.StateVector` <- function(e1, e2) {
  e1$less_than_equal(e2)
}

#' @export
`>.StateVector` <- function(e1, e2) {
  e1$greater_than(e2)
}

#' @export
`>=.StateVector` <- function(e1, e2) {
  e1$greater_than_equal(e2)
}

#' @export Snapshot
NULL

#' @export
`==.Snapshot` <- function(e1, e2) {
  e1$equal(e2)
}

#' @export
`!=.Snapshot` <- function(e1, e2) {
  e1$not_equal(e2)
}

#' @export DeleteSet
NULL

#' @export
`==.DeleteSet` <- function(e1, e2) {
  e1$equal(e2)
}

#' @export
`!=.DeleteSet` <- function(e1, e2) {
  e1$not_equal(e2)
}
