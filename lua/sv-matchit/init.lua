local M = {}

local function jump_to_match()
  local bufnr = vim.api.nvim_get_current_buf()
  if not require('sv-matchit.watcher').attach(bufnr) then
    vim.cmd('normal! %')
    return
  end

  local cursor = vim.api.nvim_win_get_cursor(0)
  local match = require('sv-matchit.rust').get_match_at(bufnr, cursor[1] - 1, cursor[2])
  if not match then
    vim.cmd('normal! %')
    return
  end

  vim.api.nvim_win_set_cursor(0, { match.mate[1] + 1, match.mate[2] })
end

--- Enable SystemVerilog keyword matching. `%` jumps between begin/end,
--- module/endmodule, and function/endfunction.
function M.setup()
  local group = vim.api.nvim_create_augroup('SvMatchit', { clear = true })
  vim.api.nvim_create_autocmd('FileType', {
    group = group,
    pattern = { 'systemverilog', 'verilog' },
    callback = function(event)
      require('sv-matchit.watcher').attach(event.buf)
      vim.keymap.set({ 'n', 'x' }, '%', jump_to_match, {
        buffer = event.buf,
        silent = true,
        desc = 'Jump to matching SystemVerilog keyword',
      })
    end,
  })

  for _, bufnr in ipairs(vim.api.nvim_list_bufs()) do
    if vim.bo[bufnr].filetype == 'systemverilog' or vim.bo[bufnr].filetype == 'verilog' then
      require('sv-matchit.watcher').attach(bufnr)
      vim.keymap.set({ 'n', 'x' }, '%', jump_to_match, { buffer = bufnr, silent = true })
    end
  end
end

return M
