#' @include extendr-wrappers.R error-wrapper.R
NULL

#' @export TextRef
NULL

#' @export
`$.TextRef` <- make_dispatcher(TextRef)

#' @export
`[[.TextRef` <- `$.TextRef`

#' @export
`$.TextEvent` <- make_dispatcher(TextEvent)

#' @export
`[[.TextEvent` <- `$.TextEvent`
