local pairs = {}

local success, err = pcall(require, 'blink.lib')
if not success then error('sv-matchit requires blink.lib ("saghen/blink.lib"): ' .. err) end

local native = require('blink.lib.native.managed').new({
  module_name = 'sv-matchit',
  library_name = 'sv_matchit',
  current_file_path = debug.getinfo(1, 'S').source:sub(2),
  logger = require('sv-matchit.logger'),
})

--- Enable SystemVerilog keyword matching.
--- @param opts? { enabled?: boolean }
function pairs.setup(opts)
  if not native:library_available() then
    error('sv-matchit native library is unavailable; run require("sv-matchit").build() before setup()')
  end
  require('sv-matchit.native_keyword').setup(opts)
end

function pairs.library_available() return native:library_available() end

function pairs.build(opts)
  return native:build(
    { 'cargo', 'build', '--release' },
    function(repo_root, platform)
      return {
        repo_root .. '/target/release/libsv_matchit' .. platform.lib_extension,
        repo_root .. '/target/release/sv_matchit' .. platform.lib_extension,
      }
    end,
    opts
  )
end

return pairs
