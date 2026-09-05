--- @class (exact) sv-matchit.ConfigStrict
--- @field mappings sv-matchit.MappingsConfig
--- @field highlights sv-matchit.HighlightsConfig
--- @field debug boolean

--- @type sv-matchit.ConfigStrict | blink.lib.Config
local config = require('blink.lib.config').new({
  mappings = require('sv-matchit.config.mappings'),
  highlights = require('sv-matchit.config.highlights'),
  debug = { false, 'boolean' },
}, { validate = false })
return config
