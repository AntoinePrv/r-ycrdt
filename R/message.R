#' @include extendr-wrappers.R error-wrapper.R
NULL

#' @export SyncMessage
NULL

#' @export Message
NULL

#' @export
`$.SyncMessage` <- make_dispatcher(SyncMessage)

#' @export
`[[.SyncMessage` <- `$.SyncMessage`

#' @export
`$.Message` <- make_dispatcher(Message)

#' @export
`[[.Message` <- `$.Message`

install_checked_static(SyncMessage, "decode_v1")
install_checked_static(SyncMessage, "decode_v2")
install_checked_static(SyncMessage, "from_sync_step1")
install_checked_static(SyncMessage, "from_sync_step2")
install_checked_static(SyncMessage, "from_update")
install_checked_static(Message, "decode_v1")
install_checked_static(Message, "decode_v2")
install_checked_static(Message, "new")

#' @export
`==.SyncMessage` <- function(e1, e2) {
  e1$equal(e2)
}

#' @export
`!=.SyncMessage` <- function(e1, e2) {
  e1$not_equal(e2)
}
