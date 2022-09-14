const init = () => Promise.resolve()
module.exports = init

const { createVal } = require('./binding')

// you can modify binding here
module.exports.createVal = createVal
