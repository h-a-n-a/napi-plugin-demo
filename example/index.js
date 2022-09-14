const { folder } = require('@node/core')
const { createVal: createVal1 } = require('@node/plugin-1')
const { createVal: createVal2 } = require('@node/plugin-2')

const val1 = createVal1()
const val2 = createVal2()

console.log(val1, val2)

const result = folder([val1, val2])
console.log(result)
