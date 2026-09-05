local watcher = { watched_bufnrs = {} }

local function parse_buffer(bufnr)
  if not vim.api.nvim_buf_is_valid(bufnr) then return false end
  local filetype = vim.bo[bufnr].filetype
  if filetype ~= 'systemverilog' and filetype ~= 'verilog' then return false end

  local lines = vim.api.nvim_buf_get_lines(bufnr, 0, -1, false)
  return require('sv-matchit.rust').parse_buffer(bufnr, filetype, lines)
end

--- Parse a SystemVerilog buffer and keep its native index current.
--- @param bufnr integer
--- @return boolean attached
function watcher.attach(bufnr)
  if watcher.watched_bufnrs[bufnr] then return true end
  if not parse_buffer(bufnr) then return false end

  watcher.watched_bufnrs[bufnr] = true
  vim.api.nvim_buf_attach(bufnr, false, {
    on_lines = function() parse_buffer(bufnr) end,
    on_reload = function() parse_buffer(bufnr) end,
    on_detach = function() watcher.watched_bufnrs[bufnr] = nil end,
  })
  return true
end

return watcher
