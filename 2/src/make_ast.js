import { create, INFY } from './values.js'

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
				id: 'main'
			},
			value: {
				name: 'main',
				data: {
					params: [
						{
							name: 'args',
							xtype: {
								id: 'arr',
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
		}
	]
}

// type obj = any?[*][2]