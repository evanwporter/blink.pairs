local mappings = {}

--- @type table<string, boolean>
local disabled_filetypes_set = {}

function mappings.enable()
  local config = require('sv-matchit.config').get()

  disabled_filetypes_set = {}
  for _, ft in ipairs(config.mappings.disabled_filetypes) do
    disabled_filetypes_set[ft] = true
  end

  require('sv-matchit.mappings.ops').register(config.mappings.pairs, config.mappings.cmdline)
  require('sv-matchit.mappings.wrap').register(config.mappings.wrap)
end

function mappings.disable()
  local config = require('sv-matchit.config').get()
  require('sv-matchit.mappings.ops').unregister(config.mappings.pairs, config.mappings.cmdline)
  require('sv-matchit.mappings.wrap').unregister(config.mappings.wrap)
end

function mappings.is_enabled()
  local mode = vim.api.nvim_get_mode().mode
  return vim.g.pairs ~= false
    and vim.b.pairs ~= false
    and vim.g.sv_matchit ~= false
    and vim.b.sv_matchit ~= false
    and mode:find('R') == nil
    and (mode ~= 'c' or (vim.fn.getcmdtype() ~= '/' and vim.fn.getcmdtype() ~= '?'))
    and not disabled_filetypes_set[vim.bo.filetype]
end

return mappings
