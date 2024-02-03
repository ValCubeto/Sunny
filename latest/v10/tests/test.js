class Iter {
  /** @type {any[]} */
  list = []
  i = 0
  constructor(list) {
    this.list = list
    this.i = 0
  }
  nextItem() {
    this.i++
    console.log({ [this.i]: this.list[this.i] })
    return this.list[this.i]
  }
  index() {
    return this.i
  }
  /** @returns {Generator<[number, any], void, unknown>} */
  [Symbol.iterator] = function*() {
    while (this.i < this.list.length) {
      yield this.list[this.i]
      this.i++
    }
  }
}

let tokens = [1, '+', 2, '*', 3]
let prior = ['*', '+']
/** @type {({ left: number, right: number, op: string })[]} */
let ast = []

const iter = new Iter(tokens)
const token = iter[Symbol.iterator]().next().value
console.log({ token })

if (typeof token === 'string') {
  throw `expected value at ${iter.index()}`
}
const left = token
const op = iter.nextItem()
if (typeof op !== 'string') {
  throw `expected op at ${iter.index()}`
}
const right = iter.nextItem()
if (typeof right === 'string') {
  throw `expexted val at ${iter.index()}`
}
ast.push({ left, right, op })

for (const token of iter.next()) {
  console.log(iter.index())
}
