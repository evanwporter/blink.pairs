use mlua::IntoLua;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Normal,
    BlockComment,
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
    matches_by_line: Vec<Vec<KeywordMatch>>,
    state_by_line: Vec<State>,
}

impl ParsedBuffer {
    pub fn supports_filetype(filetype: &str) -> bool {
        matches!(filetype, "systemverilog" | "verilog")
    }

    pub fn parse(lines: &[&str]) -> Self {
        let (matches_by_line, state_by_line) = parse_lines(lines, 0, State::Normal);
        let mut parsed = Self { matches_by_line, state_by_line };
        parsed.rematch();
        parsed
    }

    /// Replace an edited range with freshly tokenized lines. Returns whether the
    /// lexical state at the edit boundary changed; callers must then reparse the
    /// remaining buffer because it may have been inside a block comment.
    pub fn reparse_range(
        &mut self,
        lines: &[&str],
        start_line: usize,
        old_end_line: usize,
    ) -> bool {
        let start_line = start_line.min(self.matches_by_line.len());
        let old_end_line = old_end_line.min(self.matches_by_line.len());
        let initial_state = start_line
            .checked_sub(1)
            .and_then(|line| self.state_by_line.get(line))
            .copied()
            .unwrap_or(State::Normal);
        let old_end_state = old_end_line
            .checked_sub(1)
            .and_then(|line| self.state_by_line.get(line))
            .copied()
            .unwrap_or(State::Normal);
        let (new_matches, new_states) = parse_lines(lines, start_line, initial_state);
        let new_end_state = new_states.last().copied().unwrap_or(initial_state);

        self.matches_by_line.splice(start_line..old_end_line, new_matches);
        self.state_by_line.splice(start_line..old_end_line, new_states);
        self.rematch();
        old_end_state != new_end_state
    }

    pub fn match_at(&self, line: usize, col: usize) -> Option<KeywordMatch> {
        self.matches_by_line
            .get(line)?
            .iter()
            .find(|m| (m.col..m.col + m.word.len()).contains(&col) && m.mate.is_some())
            .cloned()
    }

    fn rematch(&mut self) {
        for line in &mut self.matches_by_line {
            for match_ in line {
                match_.mate = None;
            }
        }

        let mut stack: Vec<(usize, usize)> = Vec::new();
        for line in 0..self.matches_by_line.len() {
            for index in 0..self.matches_by_line[line].len() {
                let (kind, word, col) = {
                    let match_ = &self.matches_by_line[line][index];
                    (match_.kind, match_.word, match_.col)
                };
                if kind == Kind::Opening {
                    stack.push((line, index));
                    continue;
                }
                let Some(&(opening_line, opening_index)) = stack.last() else {
                    continue;
                };
                if !opening_matches(self.matches_by_line[opening_line][opening_index].word, expected_opening(word)) {
                    continue;
                }
                stack.pop();
                let opening_col = self.matches_by_line[opening_line][opening_index].col;
                if opening_line == line {
                    let matches = &mut self.matches_by_line[line];
                    matches[opening_index].mate = Some((line, col));
                    matches[index].mate = Some((opening_line, opening_col));
                } else {
                    self.matches_by_line[opening_line][opening_index].mate = Some((line, col));
                    self.matches_by_line[line][index].mate = Some((opening_line, opening_col));
                }
            }
        }
    }
}

fn parse_lines(lines: &[&str], start_line: usize, mut state: State) -> (Vec<Vec<KeywordMatch>>, Vec<State>) {
    let mut matches_by_line = Vec::with_capacity(lines.len());
    let mut state_by_line = Vec::with_capacity(lines.len());
    for (offset, text) in lines.iter().enumerate() {
        let bytes = text.as_bytes();
        let mut line_matches = Vec::new();
        let mut in_string = false;
        let mut col = 0;
        while col < bytes.len() {
            if state == State::BlockComment {
                if bytes[col..].starts_with(b"*/") {
                    state = State::Normal;
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
                state = State::BlockComment;
                col += 2;
                continue;
            }
            if bytes[col] == b'\"' {
                in_string = true;
                col += 1;
                continue;
            }
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
            if let Some((word, kind)) = keyword(&text[start..col]) {
                line_matches.push(KeywordMatch {
                    word,
                    kind,
                    line: start_line + offset,
                    col: start,
                    mate: None,
                });
            }
        }
        matches_by_line.push(line_matches);
        state_by_line.push(state);
    }
    (matches_by_line, state_by_line)
}

fn keyword(word: &str) -> Option<(&'static str, Kind)> {
    let kind = match word {
        "begin" | "module" | "function" | "task" | "class" | "package" | "interface"
        | "program" | "checker" | "primitive" | "generate" | "clocking" | "property"
        | "sequence" | "randsequence" | "specify" | "covergroup" | "config" | "fork"
        | "case" | "casex" | "casez" | "randcase" => Kind::Opening,
        "end" | "endmodule" | "endfunction" | "endtask" | "endclass" | "endpackage"
        | "endinterface" | "endprogram" | "endchecker" | "endprimitive" | "endgenerate"
        | "endclocking" | "endproperty" | "endsequence" | "endspecify" | "endgroup"
        | "endconfig" | "endcase" | "join" | "join_any" | "join_none" => Kind::Closing,
        _ => return None,
    };
    // `word` originates from this match arm, so converting it to a static
    // string documents that the Lua-facing value is one of the keywords above.
    Some((match word {
        "begin" => "begin", "module" => "module", "function" => "function", "task" => "task",
        "class" => "class", "package" => "package", "interface" => "interface", "program" => "program",
        "checker" => "checker", "primitive" => "primitive", "generate" => "generate", "clocking" => "clocking",
        "property" => "property", "sequence" => "sequence", "randsequence" => "randsequence", "specify" => "specify",
        "covergroup" => "covergroup", "config" => "config", "fork" => "fork", "case" => "case",
        "casex" => "casex", "casez" => "casez", "randcase" => "randcase", "end" => "end",
        "endmodule" => "endmodule", "endfunction" => "endfunction", "endtask" => "endtask", "endclass" => "endclass",
        "endpackage" => "endpackage", "endinterface" => "endinterface", "endprogram" => "endprogram", "endchecker" => "endchecker",
        "endprimitive" => "endprimitive", "endgenerate" => "endgenerate", "endclocking" => "endclocking", "endproperty" => "endproperty",
        "endsequence" => "endsequence", "endspecify" => "endspecify", "endgroup" => "endgroup", "endconfig" => "endconfig",
        "endcase" => "endcase", "join" => "join", "join_any" => "join_any", "join_none" => "join_none",
        _ => unreachable!(),
    }, kind))
}

fn expected_opening(closing: &str) -> &str {
    match closing {
        "end" => "begin", "endmodule" => "module", "endfunction" => "function", "endtask" => "task",
        "endclass" => "class", "endpackage" => "package", "endinterface" => "interface", "endprogram" => "program",
        "endchecker" => "checker", "endprimitive" => "primitive", "endgenerate" => "generate", "endclocking" => "clocking",
        "endproperty" => "property", "endsequence" => "sequence", "endspecify" => "specify", "endgroup" => "covergroup",
        "endconfig" => "config", "endcase" => "case", "join" | "join_any" | "join_none" => "fork",
        _ => unreachable!("only called for closing keywords"),
    }
}

fn opening_matches(opening: &str, expected: &str) -> bool {
    opening == expected
        || (expected == "case" && matches!(opening, "casex" | "casez" | "randcase"))
        || (expected == "sequence" && opening == "randsequence")
}

fn is_ident_start(byte: u8) -> bool { byte.is_ascii_alphabetic() || byte == b'_' }
fn is_ident_continue(byte: u8) -> bool { is_ident_start(byte) || byte.is_ascii_digit() || byte == b'$' }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_keywords_and_incrementally_updates_the_changed_range() {
        let mut buffer = ParsedBuffer::parse(&["module demo;", "  begin", "  end", "endmodule"]);
        assert_eq!(buffer.match_at(0, 0).unwrap().mate, Some((3, 0)));
        assert!(!buffer.reparse_range(&["  function void f;", "  endfunction"], 1, 3));
        assert_eq!(buffer.match_at(1, 2).unwrap().mate, Some((2, 2)));
        assert_eq!(buffer.match_at(0, 0).unwrap().mate, Some((3, 0)));
    }

    #[test]
    fn reports_state_changes_that_require_a_full_reparse() {
        let mut buffer = ParsedBuffer::parse(&["module demo;", "endmodule"]);
        assert!(buffer.reparse_range(&["/*"], 1, 1));
    }
}
