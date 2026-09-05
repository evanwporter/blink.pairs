local watcher = { watched_bufnrs = {} }

local function parse_buffer(bufnr, start_line, old_end_line, new_end_line)
  if not vim.api.nvim_buf_is_valid(bufnr) then return false end
  local filetype = vim.bo[bufnr].filetype
  if filetype ~= 'systemverilog' and filetype ~= 'verilog' then return false end

  local lines = vim.api.nvim_buf_get_lines(bufnr, start_line or 0, new_end_line or -1, false)
  local did_parse, state_changed = require('sv-matchit.rust').parse_buffer(
    bufnr,
    filetype,
    lines,
    start_line,
    old_end_line
  )
  if did_parse and state_changed and new_end_line then parse_buffer(bufnr) end
  return did_parse
end

--- Parse a SystemVerilog buffer and keep its native index current.
--- @param bufnr integer
--- @return boolean attached
function watcher.attach(bufnr)
  if watcher.watched_bufnrs[bufnr] then return true end
  if not parse_buffer(bufnr) then return false end

  watcher.watched_bufnrs[bufnr] = true
  vim.api.nvim_buf_attach(bufnr, false, {
    on_lines = function(_, _, _, start, old_end, new_end)
      parse_buffer(bufnr, start, old_end, new_end)
    end,
    on_reload = function() parse_buffer(bufnr) end,
    on_detach = function() watcher.watched_bufnrs[bufnr] = nil end,
  })
  return true
end

return watcher
