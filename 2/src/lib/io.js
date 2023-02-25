import { create } from '../values.js'
import { print } from '../functions.js'

function read() { /* sync code */ }

/*
read()
write(...data)
print
get_rows() -> num
get_columns() -> num
*/

const io = create.object([
	[
		'read',
		create.function({
			name: 'read',
			returns: { id: 'str' },
			value: read
		})
	],
	[
		'write',
		create.function({
			name: 'write',
			params: [
				{
					name: 'text',
					xtype: {
						id: 'str',
					}
				}
			],
		})
	],
/* 	[
		'error',
		create.function({
			name: 'error',
			params: []
		})
	], */
	[
		'print',
		create.function({
			name: 'print',
			internal: true,
			value: print
		})
	],
])

export { io }
