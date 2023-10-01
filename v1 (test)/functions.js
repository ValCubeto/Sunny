import { tokenize } from './tokenize.js'
import { make_ast } from './make_ast.js'
import { exec_ast } from './exec_ast.js'
import { stdout } from 'node:process'
import { EOL } from 'node:os'
import { to } from './to.js'
import { id } from './identifiers.js'

export const functions = {
	'eval': function (code, isMain = false) {
		console.log(`[debug] executing`, { isMain, code })
		const tokens = tokenize(code)
		const ast = make_ast(tokens)
		exec_ast(ast, isMain)
	},
	'print': function (values, sep = ' ', end = EOL) {
		if (!Array.isArray(values)) {
			console.log('InternalError: el primer argumento de print debe ser un array wwwwwwwwwwwwwwww')
		}
		for (const i in values) {
			stdout.write(repr(values[i]))
			if (i < values.length - 1) {
				stdout.write(repr(sep))
			}
		}
		stdout.write(repr(end))
	}
}

function repr(value) {
	if (typeof value === 'object' && id.type in value) {
		return to.str(value)[id.value]
	}
	return typeof value === 'object' ? `<JS object ${value.constructor.name}>` : `${value}`
}

// functions.print(['hola', 'chau'].map(to.str))