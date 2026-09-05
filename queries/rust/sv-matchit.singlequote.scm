[
  (parameters)
  (type_arguments)
  (type_parameters)
  (reference_type)
] @nopair.inside

[
  (type_identifier)
  (reference_type)
] @nopair.inside_or_after

; struct Foo<
; fn foo<
; impl<
(ERROR
  .
  [
    "struct"
    "fn"
    "impl"
  ]) @nopair.inside_or_after

; emtpy function return type
; fn foo() -> {}
"->" @nopair.after
