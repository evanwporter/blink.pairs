# SV Matchit

Native `%` matching for SystemVerilog block keywords in Neovim.

| Opening | Closing |
| --- | --- |
| `begin` | `end` |
| `module` | `endmodule` |
| `function` | `endfunction` |

Keywords in comments, strings, escaped identifiers, and longer identifiers are
ignored. The buffer is reparsed after each edit; `%` jumps to the matching
keyword when the cursor is on one, and otherwise retains Neovim's normal `%`
behavior.

## Install

Build the native library in the repository before starting Neovim:

```sh
cargo build --release
```

The plugin loads the resulting library directly from `target/release`; it does
not download, build, or search Neovim's runtime path for it.

```lua
{
  dir = '~/sv-matchit',
  name = 'systemverilog-keyword-pairs',
  config = function() require('sv-matchit').setup() end,
}
```
