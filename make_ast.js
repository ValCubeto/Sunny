import { id } from './identifiers.js'

export function make_ast(tokens) {
	const ast = []
	for (const token of tokens) {
		//
	}
	// return ast
	return [
		{
			[id.type]: id.declare,
			[id.data]: {
				[id.name]: 'main',
			},
			[id.value]: {
				[id.type]: id.function,
				[id.value]: [
					{
						[id.type]: id.call,
						[id.data]: {
							[id.name]: 'print',
						},
						[id.value]: [
							{
								[id.type]: id.string,
								[id.value]: 'Hello world!'
							}
						]
					}
				]
			}
		}
	]
}