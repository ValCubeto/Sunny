import { EOL } from 'node:os'
import { key_throw } from './keys.js'
import { tokenize } from './tokenize.js'
import { make_ast } from './make_ast.js'
import { exec_ast } from './exec_ast.js'
import { g } from './values.js'

export function print(values, sep, end) {
	if (!Array.isArray(values)) {
		key_throw(`InternalError: invalid first argument passed to function 'print'`)
	}
	process.stdout.write(values.join(sep ?? ' '))
	process.stdout.write(end ?? EOL)
}

export function eval_code(code, expression, internal, main) {
	const tokens = tokenize(code)
	const ast = make_ast(tokens)
	return exec_ast(ast, [g], expression, internal, main)
}