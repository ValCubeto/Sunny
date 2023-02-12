import { print } from './functions.js'
import { NAME, VERSION } from './info.js'

export function key_throw(value) {
	console.log(`[throwing]`)
	print([value])
	print([])
	print([`${NAME} v${VERSION}`])
	process.exit(1)
}