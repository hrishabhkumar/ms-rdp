const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'ms-rdp' means native addon name is `ms-rdp`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `ms-rdp.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `ms-rdp-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'ms-rdp', 'ms-rdp')
