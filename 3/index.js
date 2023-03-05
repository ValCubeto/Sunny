import { existsSync, readFileSync, statSync } from 'node:fs'
import { join as join_paths } from 'node:path'
import { exit } from 'node:process'

/**
 * @param {string[]} args 
 */
function main(args) {
	const executor_path = args[0]
	const flags = []
	// let expected_flag = ''
	let args_index = 0
	let code = ''
	let file = '' // '<eval>'
	for (args_index = 1; args_index < args.length; args_index++) {
		if (args[args_index][0] !== '-') break
		if (args[args_index] === '-h' || args[args_index] === '--help') {
			display_help()
			return
		}
		flags.push(args[args_index])
		/* const arg = args[i]
		if (expected_flag !== '') {
			exec_flags[expected_flag] = arg
			expected_flag = ''
			continue
		}
		if (arg[0] !== '-' || arg.length < 2) {
			break
		}
		if (arg[1] !== '-') {
			if (!ALPHA.includes(arg[1])) {
				fail(`Invalid flag at position ${i}`)
			}
			if (!SHORT_FLAGS.includes(arg[1])) {
				fail(`Unknown flag: ${arg}`)
			}
			if (arg[1] === 'h') {
				display_help()
				return 0
			}
			if (arg[1] === 'e') {
				from_cmd_line = true
			}
			if (SHORT_FLAGS_WITH_VALUES.includes(arg[1])) {
				expected_flag = arg
				continue
			}
			exec_flags[arg[1]] = 'y'
			continue
		}
		if (arg.length < 3) {
			fail(`What did you mean with ${arg} lol`)
		}
		if (arg.length > 21) {
			fail(`Flag name too long: ${arg}`)
		}
		const flag = arg.slice(2).split('=', 1)
		const flag_name = flag[0]
		if (!FLAGS.includes(flag_name)) {
			fail(`Unknown flag: --${flag_name}`)
		}
		if (FLAGS_WITH_VALUES.includes(flag_name) && flag.length < 2) {
			fail(`Missing flag value for --${flag_name}`)
		}
		if (flag_name === 'help') {
			display_help()
			return 0
		}
		if (flag_name === 'eval') {
			from_cmd_line = true
			exec_flags['e'] = flag[1]
		}
	}
	if (expected_flag) {
		fail(`Missing flag value for -${expected_flag}`) */
	}

	if (code === '') {
		if (args_index >= args.length) {
			fail('Missing file path')
		}
		const relative_path = args[args_index]

		const path = join_paths(process.cwd(), relative_path)
		const has_extension = relative_path.length >= EXTENSION.length && relative_path.slice(-EXTENSION.length).toLowerCase() === EXTENSION
		if (has_extension) {
			[code, file] = try_read([path])
		} else {
			[code, file] = try_read([path, path + EXTENSION])
		}
	}
	const exec_args = args.slice(args_index + 1)
	console.log('[debug]', { executor_path, /* exec_ */flags, exec_args, code })

	function get_token_type(char) {
		if (char === ' ')
			return TokenType.Space
		if (char === '\t')
			return TokenType.Tab
		if (char === '\n')
			return TokenType.Eol
		if (DIGITS.includes(char))
			return TokenType.Number
		if (IDENTIFIER_CHARS.includes(char))
			return TokenType.Word
		if (QUOTES.includes(char))
			return TokenType.String
		if (OPERATOR_CHARS.includes(char))
			return TokenType.Operator
		fail(`SyntaxError: unexpected char \\${char.charCodeAt(0).toString(16)}\n\tat ${file}:${line}:${column}`)
	}

	let line = 0
	let column = 0
	let tokens = []
	let token_text = code[0]
	let token_type = get_token_type(token_text)
	let space_count = TokenType.Eol === token_type ? 0 : 1

	for (let i = 1; i < code.length; i++) {
		const char = code[i]
		if (token_type === TokenType.None) {
			token_type = get_token_type(char)
			token_text = char
			console.log({ line, column, token_type })
			continue
		}
		if (token_type === TokenType.Space) {}
		console.log('ignored', char)
	}
}

const TokenType = {
	None: 0,
	Space: 4,
	Tab: 2,
	Eol: 3,
	Word: 1,
	Number: 5,
	String: 6,
	// Regexp: 7,
	Operator: 8
}

function fail(message) {
	console.error(message)
	exit(1)
}

const PROGRAM_NAME = 'Sunny'
const VERSION = '1.0'
const EXTENSION = '.sny'

function display_help() {
	const text = readFileSync('./help.txt').toString()
	console.log(text)
}

const FLAGS = [
	'help',
	'eval',
	'test',
]

const SHORT_FLAGS = [
	'h',
	'e',
	't',
]

const FLAGS_WITH_VALUES = [
	'eval'
]

const SHORT_FLAGS_WITH_VALUES = [
	'e'
]

const ALPHA_LOWER = 'abcdefghijklmnopqrstuvwxyz'
const ALPHA_UPPER = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
const ALPHA = ALPHA_LOWER + ALPHA_UPPER
const IDENTIFIER_CHARS = ALPHA_LOWER + '_' + ALPHA_UPPER
const DIGITS = '1234567890'
const QUOTES = `'"`
const OPERATOR_CHARS = '+-*/'
const OPERATORS = [
	'+',
	'-',
	'*',
	'/',
	'**'
]
const KEYWORDS = [
	'if', 'else',
	'match',
	'var', 'const',
	'class', 'fun',
	'import', 'as', 'from'
]

function try_read(paths) {
	for (const path of paths) {
		if (!existsSync(path)) {
			continue
		}
		if (!statSync(path).isFile()) {
			fail(`"${paths[0]}" is not a file`)
		}
		try {
			const file = readFileSync(path).toString()
			return [file, path]
		} catch (error) {
			fail(`Failed to read "${path}" (code: ${error.code})`)
		}
	}
	fail(`File "${paths[0]}" not found`)
}

main(process.argv.slice(1))