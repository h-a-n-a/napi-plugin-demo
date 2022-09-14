const { folder } = require('@node/core')
const { createPlugin: createPlugin1 } = require('@node/plugin-1')
const { createPlugin: createPlugin2 } = require('@node/plugin-2')

const val1 = createPlugin1()
const val2 = createPlugin2()

console.log(val1, val2)

const result = folder([val1, val2])
console.log(result)
