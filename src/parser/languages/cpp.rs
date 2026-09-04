use crate::parser::*;
use matcher_macros::define_matcher;

// TODO: Does not support arbitrary delimiters: R"tag(contents)tag"

define_matcher!(Cpp {
    delimiters: [
        "(" => ")",
        "[" => "]",
        "{" => "}"
    ],
    line_comment: ["//"],
    block_comment: ["/*" => "*/"],
    char: ["'"],
    string: ["\""],
    block_string: ["R\"(" => ")\""]
});
