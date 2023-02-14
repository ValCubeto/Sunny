import { key_throw } from './keys.js'

export function tokenize(chars) {
	const tokens = []
	let token = chars[0]
	let type = token_type(chars[0])
	let start = 0
	let acc = 'none'
	let escaping = false
	for (let i = 1; i < chars.length; i++) {
		const char = chars[i]
		console.log({ i })
		if (char === ' ') {
			//if ()
			tokens.push({ type, text: token, start })
			token = ''
			type = TOKEN_TYPES.Unknown
			console.log(tokens.at(-1))
			continue
		}
		if (type !== token_type(char)) {
			tokens.push({ type, text: token, start })
			start = i
			token = char
			type = token_type(char)
			console.log(tokens.at(-1))
			continue
		}
		token += char
	}
	console.log(tokens)
	// return tokens
	return [
		{
			text: 'fun',
			type: 'word',
			start: 0
		}
	]
}

function token_type(char) {
	return (
		char === '\t'
			? TOKEN_TYPES.Tab
		: WORD_CHARS.includes(char)
			? TOKEN_TYPES.Word
		: OTHER_CHARS.includes(char)
			? TOKEN_TYPES.Other
		: BRACKETS.includes(char)
			? TOKEN_TYPES.Bracket
		: char === '\n'
			? TOKEN_TYPES.Eol
		: SYMBOL_CHARS.includes(char)
			? TOKEN_TYPES.Symbol
		: NUMBER_CHARS.includes(char)
			? TOKEN_TYPES.Number
		: -1//key_throw(`SyntaxError: invalid character (${char} - ${char.charCodeAt(0)})`)
	)
}

const WORD_CHARS = 'abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ'
const NUMBER_CHARS = '0123456789'
const SYMBOL_CHARS = '.,:+-*/%<>'
const OTHER_CHARS = `.,:'"`

const SYMBOLS = [
	'+',
	'-',
	'*',
	'/',
	'%',
	'**',
	'<',
	'>',
]
const BRACKETS = [
	'(',
	')',
	'[',
	']',
	'{',
	'}',
]

const TOKEN_TYPES = {
	Unknown: 'unknown',
	Tab    : 'tab',
	Word   : 'word',
	Other  : 'other',
	Bracket: 'bracket',
	Eol    : 'eol',
	String : 'string',
	Regexp : 'regexp',
	Symbol : 'symbol',
	Number : 'number',
}