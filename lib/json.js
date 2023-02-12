import { to } from '../to.js'
import { id } from '../identifiers.js'

export const json = to.obj({
	jsonify: to.Function(
		function ({ value, indent }) {
			return to.str(JSON.stringify(value, null, indent))
		},
		{
			[id.params]: [
				to.param('value'),
				to.param('indent', null, { [id.optional]: true })
			],
			[id.returns]: to.str()
		}
	),
	parse: to.Function(
		function ({ string }) {
			return {
				[id.value]: JSON.parse(string)
			}
		},
		{
			[id.params]: [
				to.param('string', id.string)
			]
		}
	)
})