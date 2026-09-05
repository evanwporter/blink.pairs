local function set_highlights()
  vim.api.nvim_set_hl(0, 'SvMatchitOrange', { ctermfg = 15, fg = '#d65d0e', default = true })
  vim.api.nvim_set_hl(0, 'SvMatchitPurple', { ctermfg = 13, fg = '#b16286', default = true })
  vim.api.nvim_set_hl(0, 'SvMatchitBlue', { ctermfg = 12, fg = '#458588', default = true })
  vim.api.nvim_set_hl(0, 'SvMatchitUnmatched', { ctermfg = 9, fg = '#ff007c', default = true })
  vim.api.nvim_set_hl(0, 'SvMatchitMatchParen', { link = 'MatchParen', default = true })
end

set_highlights()
vim.api.nvim_create_autocmd('ColorScheme', {
  group = vim.api.nvim_create_augroup('sv_matchit_highlights', {}),
  callback = set_highlights,
})
