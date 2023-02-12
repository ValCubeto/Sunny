import { functions } from './functions.js'

export const keys = {
	// operators
	'add': function (a, b) {},
	'sub': function (a, b) {},
	'mul': function (a, b) {},
	'div': function (a, b) {},
	'pow': function (a, b) {},
	'rest': function (a, b) {},
	'nullish': function (a, b) {},

	// keywords
	'throw': function (value) {
		// si usas console.log se muestra el objeto tal cuál, w
		functions.print([value])
		process.exit(1)
	}
}