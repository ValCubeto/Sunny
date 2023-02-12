import { EOL } from 'node:os'
import { key_throw } from './keys.js'

export function print(values, sep, end) {
	if (!Array.isArray(values)) {
		key_throw(`InternalError: invalid first argument passed to function 'print'`)
	}
	process.stdout.write(values.join(sep ?? ' '))
	process.stdout.write(end ?? EOL)
}