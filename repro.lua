vim.pack.add({
  'https://github.com/saghen/blink.download',
  { src = 'https://github.com/saghen/blink.pairs', version = vim.version.range('*') },
})
require('blink.pairs').build():pwait(60000)
require('blink.pairs').setup()
