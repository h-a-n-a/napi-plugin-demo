const init = () => Promise.resolve()
module.exports = init

const { createPlugin } = require('./binding')

// you can modify binding here
module.exports.createPlugin = createPlugin
