#' @include extendr-wrappers.R error-wrapper.R
NULL

#' @export MapRef
NULL

#' @export
`$.MapRef` <- make_dispatcher(MapRef)

#' @export
`[[.MapRef` <- `$.MapRef`

#' @export
`$.MapEvent` <- make_dispatcher(MapEvent)

#' @export
`[[.MapEvent` <- `$.MapEvent`
