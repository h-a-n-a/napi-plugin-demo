const init = () => Promise.resolve()
module.exports = init

const { folder } = require('./binding')

// you can modify binding here
module.exports.folder = folder
