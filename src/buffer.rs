use mlua::IntoLua;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Opening,
    Closing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordMatch {
    pub word: &'static str,
    pub kind: Kind,
    pub line: usize,
    pub col: usize,
    pub mate: Option<(usize, usize)>,
}

impl IntoLua for KeywordMatch {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("word", self.word)?;
        table.set("line", self.line)?;
        table.set("col", self.col)?;
        if let Some((line, col)) = self.mate {
            table.set("mate", lua.create_sequence_from([line, col])?)?;
        }
        (&table).into_lua(lua)
    }
}

pub struct ParsedBuffer {
    matches: Vec<KeywordMatch>,
}

impl ParsedBuffer {
    pub fn supports_filetype(filetype: &str) -> bool {
        matches!(filetype, "systemverilog" | "verilog")
    }

    pub fn parse(lines: &[&str]) -> Self {
        let mut matches = Vec::new();
        let mut stack: Vec<usize> = Vec::new();
        let mut in_block_comment = false;
        let mut in_string = false;

        for (line, text) in lines.iter().enumerate() {
            let bytes = text.as_bytes();
            let mut col = 0;
            while col < bytes.len() {
                if in_block_comment {
                    if bytes[col..].starts_with(b"*/") {
                        in_block_comment = false;
                        col += 2;
                    } else {
                        col += 1;
                    }
                    continue;
                }
                if in_string {
                    if bytes[col] == b'\\' {
                        col += 2;
                    } else if bytes[col] == b'\"' {
                        in_string = false;
                        col += 1;
                    } else {
                        col += 1;
                    }
                    continue;
                }
                if bytes[col..].starts_with(b"//") {
                    break;
                }
                if bytes[col..].starts_with(b"/*") {
                    in_block_comment = true;
                    col += 2;
                    continue;
                }
                if bytes[col] == b'\"' {
                    in_string = true;
                    col += 1;
                    continue;
                }
                // SystemVerilog escaped identifiers continue through the next whitespace.
                if bytes[col] == b'\\' {
                    col += 1;
                    while col < bytes.len() && !bytes[col].is_ascii_whitespace() {
                        col += 1;
                    }
                    continue;
                }
                if !is_ident_start(bytes[col]) {
                    col += 1;
                    continue;
                }

                let start = col;
                col += 1;
                while col < bytes.len() && is_ident_continue(bytes[col]) {
                    col += 1;
                }
                let word = &text[start..col];
                let (kind, expected_opening) = match word {
                    "begin" => (Kind::Opening, ""),
                    "module" => (Kind::Opening, ""),
                    "function" => (Kind::Opening, ""),
                    "end" => (Kind::Closing, "begin"),
                    "endmodule" => (Kind::Closing, "module"),
                    "endfunction" => (Kind::Closing, "function"),
                    _ => continue,
                };
                let word: &'static str = match word {
                    "begin" => "begin",
                    "module" => "module",
                    "function" => "function",
                    "end" => "end",
                    "endmodule" => "endmodule",
                    "endfunction" => "endfunction",
                    _ => unreachable!(),
                };

                let idx = matches.len();
                matches.push(KeywordMatch { word, kind, line, col: start, mate: None });
                if kind == Kind::Opening {
                    stack.push(idx);
                } else if let Some(&opening_idx) = stack.last()
                    && matches[opening_idx].word == expected_opening
                {
                    stack.pop();
                    let opening = (matches[opening_idx].line, matches[opening_idx].col);
                    let closing = (line, start);
                    matches[opening_idx].mate = Some(closing);
                    matches[idx].mate = Some(opening);
                }
            }
            // Strings do not continue across ordinary SystemVerilog source lines.
            in_string = false;
        }
        Self { matches }
    }

    pub fn match_at(&self, line: usize, col: usize) -> Option<KeywordMatch> {
        self.matches
            .iter()
            .find(|m| m.line == line && (m.col..m.col + m.word.len()).contains(&col) && m.mate.is_some())
            .cloned()
    }
}

fn is_ident_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_ident_continue(byte: u8) -> bool {
    is_ident_start(byte) || byte.is_ascii_digit() || byte == b'$'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_nested_keywords_and_ignores_non_code() {
        let buffer = ParsedBuffer::parse(&[
            "module demo; // begin",
            "  begin",
            "    function void f; string s = \"endfunction\"; endfunction",
            "  end",
            "endmodule",
        ]);
        assert_eq!(buffer.match_at(0, 0).unwrap().mate, Some((4, 0)));
        assert_eq!(buffer.match_at(1, 2).unwrap().mate, Some((3, 2)));
        assert_eq!(buffer.match_at(2, 4).unwrap().mate, Some((2, 47)));
        assert!(buffer.match_at(0, 16).is_none());

        let ignored = ParsedBuffer::parse(&[
            "/* begin */",
            "\\begin end",
            "beginning endmodule_suffix",
        ]);
        assert!(ignored.match_at(0, 3).is_none());
        assert!(ignored.match_at(1, 1).is_none());
        assert!(ignored.match_at(2, 0).is_none());
    }
}
