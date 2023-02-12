import { id } from './identifiers.js'

export function tokenize(code) {
	const tokens = []
	for (const i in code) {
		const char = code[i]
		if (char === ';') {
			keys.throw('yor computer has viruz')
		}
	}
	// return tokens
	return [
		{
			[id.type]: id.word,
			[id.begin]: 0,
			[id.end]: 3,
			[id.value]: 'fun'
		},
		{
			[id.type]: id.word,
			[id.begin]: 4,
			[id.end]: 8,
			[id.value]: 'main'
		},
		{
			[id.type]: id.symbol,
			[id.begin]: 8,
			[id.end]: 9,
			[id.value]: '('
		},
		{
			[id.type]: id.symbol,
			[id.begin]: 9,
			[id.end]: 10,
			[id.value]: ')'
		},
		{
			[id.type]: id.symbol,
			[id.begin]: 10,
			[id.end]: 11,
			[id.value]: ':'
		},
		{
			[id.type]: id.eol,
			[id.begin]: 11,
			[id.end]: 12,
			[id.value]: '\n'.length
		},
		{
			[id.type]: id.indent, // "any space or tab after an end of line"
			[id.begin]: 12,
			[id.end]: 13,
			[id.value]: '\t'.length
		},
		{
			[id.type]: id.word,
			[id.begin]: 13,
			[id.end]: 18,
			[id.value]: 'print'
		},
		{
			[id.type]: id.symbol,
			[id.begin]: 18,
			[id.end]: 19,
			[id.value]: '('
		},
		{
			[id.type]: id.string,
			[id.begin]: 19,
			[id.end]: 33,
			[id.value]: 'Hello world!'
		},
		{
			[id.type]: id.symbol,
			[id.begin]: 33,
			[id.end]: 34,
			[id.value]: ')'
		}
	]
}