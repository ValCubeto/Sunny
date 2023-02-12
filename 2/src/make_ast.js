import { create, INFY } from './values.js'
import { io } from './lib/io.js'
export function make_ast(tokens) {
	const ast = []
	for (const i in tokens) {
		//
	}
	// return ast
	return [
		{
			type: 'declare',
			data: {
				type: 'function',
				name: 'main'
			},
			value: {
				name: 'main',
				params: [
					{
						name: 'args',
						xtype: {
							name: 'arr',
							model: [
								{
									name: 'I',
									value: {
										ref: 'str'
									}
								},
								{
									name: 'size',
									value: create.range(create.number('0'), INFY)
								}
							]
						}
					}
				]
			}
		}
	]
}