#' @include extendr-wrappers.R error-wrapper.R
NULL

#' @export ArrayRef
NULL

#' @export
`$.ArrayRef` <- make_dispatcher(ArrayRef)

#' @export
`[[.ArrayRef` <- `$.ArrayRef`

#' @export
`$.ArrayEvent` <- make_dispatcher(ArrayEvent)

#' @export
`[[.ArrayEvent` <- `$.ArrayEvent`
