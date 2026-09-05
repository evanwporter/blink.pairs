--- @class sv-matchit.Parser
--- @field parse_buffer fun(bufnr: number?, shiftwidth: number, filetype: string, lines: string[], start_line: number?, old_end_line: number?, new_end_line: number?): boolean
--- @field supports_filetype fun(filetype: string): boolean
--- @field get_line_matches fun(bufnr: number, line_number: number, token_type: number?): sv-matchit.Match[]
--- @field get_span_at fun(bufnr: number, row: number, col: number): string?
--- @field get_match_at fun(bufnr: number, row: number, col: number): sv-matchit.Match?
--- @field get_match_pair fun(bufnr: number, row: number, col: number): sv-matchit.MatchWithLine[]?
--- @field get_surrounding_match_pair fun(bufnr: number, row: number, col: number): sv-matchit.MatchWithLine[]?
--- @field get_unmatched_opening_before fun(bufnr: number, opening: string, closing: string, row: number, col: number): sv-matchit.MatchWithLine?
--- @field get_unmatched_closing_after fun(bufnr: number, opening: string, closing: string, row: number, col: number): sv-matchit.MatchWithLine?
--- @field get_unterminated_opening_before fun(bufnr: number, opening: string, row: number, col: number): sv-matchit.MatchWithLine?
--- @field get_unterminated_opening_after fun(bufnr: number, opening: string, row: number, col: number): sv-matchit.MatchWithLine?

--- @class sv-matchit.Match
--- @field [1] string
--- @field [2] string?
--- @field span string?
--- @field col number
--- @field stack_height number?

--- @class sv-matchit.MatchWithLine : sv-matchit.Match
--- @field line number

local project_root = vim.fn.fnamemodify(debug.getinfo(1, 'S').source:sub(2), ':h:h:h:h')
local native = require('blink.lib.native')
--- @type sv-matchit.Parser
local rust = native.load('sv_matchit', native.try_git_commit(project_root))
return rust
