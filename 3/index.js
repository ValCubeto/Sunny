import { existsSync, readFileSync, statSync } from 'node:fs'
import { join as join_paths } from 'node:path'
import { exit } from 'node:process'

/**
 * @param {string[]} args 
 */
function main(args) {
	const executor_path = args[0]
	const exec_flags = {}
	let expected_flag = ''
	let i = 0
	let from_cmd_line = false
	for (i = 1; i < args.length; i++) {
		if (args[i][0] !== '-') break
		if (args[i] === '-h' || args[i] === '--help') {
			display_help()
			return
		}
		exec_flags.push()
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

	let code = ''
	if (from_cmd_line) {
		code = exec_flags['e']
	} else {
		if (i >= args.length) {
			fail('Missing file path')
		}
		const file_relative_path = args[i]

		const path = join_paths(process.cwd(), file_relative_path)
		const has_extension = file_relative_path.length >= EXTENSION.length && file_relative_path.slice(-EXTENSION.length).toLowerCase() === EXTENSION
		code = try_read(has_extension ? [path] : [path, path + EXTENSION])
	}
	const exec_args = args.slice(i + 1)
	console.log({ executor_path, exec_flags, exec_args, code })
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
const NUMBER_CHARS = DIGITS + `.'e-+`
const OPERATORS = '+-*/'
const KEYWORDS = [
	'if', 'else',
	'var', 'const',
	'class', 'fun',
	'import'
]

function try_read(paths) {
	for (const path of paths) {
		if (!existsSync(path)) {
			continue
		}
		if (!statSync(path).isFile()) {
			console.log(`"${paths[0]}" is not a file`)
			exit(1)
		}
		try {
			const file = readFileSync(path).toString()
			return file
		} catch (error) {
			fail(`Failed to read "${path}" (code: ${error.code})`)
		}
	}
	console.log(`File "${paths[0]}" not found`)
	exit(1)
}

main(process.argv.slice(1))