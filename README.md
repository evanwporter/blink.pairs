<p align="center">
  <h2 align="center">SV Matchit (sv-matchit)</h2>
</p>

<p align="center">
	Intelligent auto-pairs with rainbow highlighting for Neovim
</p>

**sv-matchit** combines auto-pairs with rainbow highlighting, via a custom internal parser. The parser takes ~2ms to parse a 400k character file, and ~0.15ms for incremental updates. It uses indent-aware matching of delimiters and highlights mismatched pairs. See [the roadmap](https://github.com/evanwporter/sv-matchit/issues/9) for the current status, contributions welcome!

- Works out of the box with no additional configuration
- Rainbow highlighting of pairs
- Indent-aware matching of delimiters to highlight mismatched pairs
- Wrapping via motions (dot-repeatable) or treesitter
- Updates on every keystroke (<0.2ms)

<img width="900" src="https://github.com/user-attachments/assets/754479b0-1820-4fdb-b205-cbeafe894778" alt="Screenshot" />

*code from [frizbee](https://github.com/saghen/frizbee), font [iosevka](https://github.com/be5invis/Iosevka), theme [one-dark](https://github.com/navarasu/onedark.nvim) with catppuccin bg*

## Behavior

The behavior was inspired by [lexima.vim](https://github.com/cohama/lexima.vim) and [nvim-autopairs](https://github.com/windwp/nvim-autopairs)

| Before   | Input   | After    |
|----------|---------|----------|
| `\|`       | `(`       | `(\|)`     |
| `\|)`      | `(`       | `(\|)`     |
| `\|`       | `"`       | `"\|"`     |
| `""\|`     | `"`       | `"""\|"""` |
| `"foo\|`   | `"`       | `"foo"\|`  |
| `\|foo"`   | `"`       | `"\|foo"`  |
| `''\|`     | `'`       | `'''\|'''` |
| `\\|`       | `[`       | `\[\|`     |
| `\\|`       | `"`       | `\"\|`     |
| `\\|`       | `'`       | `\'\|`     |
| `A`        | `'`       | `A'`       |
| `(\|)`     | `)`       | `()\|`     |
| `((\|)`     | `)`       | `(()\|)`     |
| `'\|'`     | `'`       | `''\|`     |
| `'''\|'''` | `'`       | `''''''\|` |
| `(\|)`     | `<BS>`    | `\|`       |
| `'\|'`     | `<BS>`    | `\|`       |
| `( \| )`   | `<BS>`    | `(\|)`     |
| `(\|)`     | `<Space>` | `( \| )`   |
| `foo(\|)'bar'`     | `<C-b>aq` | `foo('bar'\|)`   |

## Installation

```lua
{
  'evanwporter/sv-matchit',
  dependencies = 'saghen/blink.lib',

  version = '*',
  -- download prebuilt binaries from github releases, must be on a versioned release
  build = function() require('sv-matchit').download():pwait(60000) end,
  -- OR build from source
  -- build = function() require('sv-matchit').build():pwait(60000) end,

  --- @module 'sv-matchit'
  --- @type sv-matchit.Config
  opts = {
    mappings = {
      -- you can call require("sv-matchit.mappings").enable()
      -- and require("sv-matchit.mappings").disable()
      -- to enable/disable mappings at runtime
      enabled = true,
      cmdline = true,
      -- or disable with `vim.g.pairs = false` (global) and `vim.b.pairs = false` (per-buffer)
      -- and/or with `vim.g.sv_matchit = false` and `vim.b.sv_matchit = false`
      disabled_filetypes = {},
      wrap = {
        -- move closing pair via motion
        ['<C-b>'] = 'motion',
        -- move opening pair via motion
        ['<C-S-b>'] = 'motion_reverse',
        -- set to 'treesitter' or 'treesitter_reverse' to use treesitter instead of motions
        -- set to nil, '' or false to disable the mapping
        -- normal_mode = {} <- for normal mode mappings, only supports 'motion' and 'motion_reverse'
      },
      -- see the defaults:
      -- https://github.com/evanwporter/sv-matchit/blob/main/lua/sv-matchit/config/mappings.lua#L52
      pairs = {},
    },
    highlights = {
      enabled = true,
      -- requires require('vim._core.ui2').enable({}), otherwise has no effect
      cmdline = true,
      -- set to { 'SvMatchit' } to disable rainbow highlighting
      groups = { 'SvMatchitOrange', 'SvMatchitPurple', 'SvMatchitBlue' },
      unmatched_group = 'SvMatchitUnmatched',

      -- highlights matching pairs under the cursor
      matchparen = {
        enabled = true,
        -- known issue where typing won't update matchparen highlight, disabled by default
        cmdline = false,
        -- also include pairs not on top of the cursor, but surrounding the cursor
        include_surrounding = false,
        group = 'SvMatchitMatchParen',
        priority = 250,
      },
    },
    debug = false,
  }
}
```

### `vim.pack`

```lua
vim.pack.add({
  'https://github.com/saghen/blink.lib',
  { src = 'https://github.com/evanwporter/sv-matchit', version = vim.version.range('*') },
})

-- download prebuilt binaries from github releases, must be on a versioned release
require('sv-matchit').download():pwait(60000)
-- OR build from source
-- require('sv-matchit').build():pwait(60000)

-- see above for the config
require('sv-matchit').setup()
```
