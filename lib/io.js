import { id } from '../identifiers.js'
import { to } from '../to.js'
import { stdout } from 'node:process'

function read() {}

const io = to.obj({
	'write': to.Function(
		function ({ values }) {
			write(values[id.value])
		},
		{
			[id.internal]: true,
			[id.params]: [
				to.param(
					'values',
					{
						[id.name]: 'arr',
						[id.model]: {
							'V': 'str'
						}
					}, 
					{
						[id.rest]: true,
					}
				)
			]
		}
	),
	'read': to.Function(read, {
		[id.internal]: true,
		[id.returns]: {
			[id.name]: 'str'
		}
	})
})

export { io }

function write(values) {
	stdout.write(values.join(''))
}

console.log(JSON.stringify({ io }, null, 2))