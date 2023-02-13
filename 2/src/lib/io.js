import { create } from '../values.js'
import { print } from '../functions.js'

const io = create.object([
	['read', create.function('read')],
	['write', create.function('write')],
	['error', create.function('error')],
	['print', print],
])

export { io }
