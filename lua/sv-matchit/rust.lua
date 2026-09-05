--- @class sv-matchit.KeywordMatch
--- @field word string
--- @field line integer Zero-based line number
--- @field col integer Zero-based byte column
--- @field mate integer[] { zero-based line number, zero-based byte column }

--- @class sv-matchit.Parser
--- @field parse_buffer fun(bufnr: integer, filetype: string, lines: string[]): boolean
--- @field get_match_at fun(bufnr: integer, row: integer, col: integer): sv-matchit.KeywordMatch?

local project_root = vim.fn.fnamemodify(debug.getinfo(1, 'S').source:sub(2), ':h:h:h')
local sysname = vim.uv.os_uname().sysname
local extension = sysname == 'Darwin' and '.dylib' or (sysname:match('Windows') and '.dll' or '.so')
local prefix = sysname:match('Windows') and '' or 'lib'
local library = project_root .. '/target/release/' .. prefix .. 'sv_matchit' .. extension

local loader, err = package.loadlib(library, 'luaopen_sv_matchit')
if not loader then error(('sv-matchit could not load %s; run `cargo build --release`: %s'):format(library, err)) end
return loader()
