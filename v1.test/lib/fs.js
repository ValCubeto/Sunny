import { readFileSync, writeFileSync } from 'node:fs'
import { id } from '../identifiers.js'
import { to } from '../to.js'
import { print } from '../functions.js'

export const fs = to.obj({
	'read_file': to.Function(
		function ({ path, encoding = 'utf8' }) {
			try {
				return readFileSync(path, { encoding }).toString()
			} catch (error) {
				print([to.obj({ message, code: error.code })])
			}
		},
		{
			[id.internal]: true,
			[id.params]: [
				to.param('path', id.string),
				to.param('encoding', id.string, {
					[id.optional]: true,
					[id.xtype]: {
						[id.match]: ['utf8']
					},
					[id.default]: 0
				}),
			]
		}
	),
	'write_file': to.Function(
		function ({ path, data, encoding = 'utf8' }) {
			writeFileSync(path, data, { encoding })
		},
		{
			[id.internal]: true,
			[id.params]: [
				to.param('path')
			]
		}
	)
})