vim.pack.add({
  'https://github.com/saghen/blink.download',
  { src = 'https://github.com/evanwporter/sv-matchit', version = vim.version.range('*') },
})
require('sv-matchit').build():pwait(60000)
require('sv-matchit').setup()
