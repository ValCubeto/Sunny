import { readFileSync } from 'node:fs'

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
		const arg = args[i]
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
				console.log(`Invalid flag at position ${i}`)
				return 1
			}
			if (!SHORT_FLAGS.includes(arg[1])) {
				console.log(`Unknown flag: ${arg}`)
				return 1
			}
			if (arg[1] === 'h') {
				display_help()
				return 0
			}
			if (arg[1] === 'e') {
				from_cmd_line = true
			}
			if (SHORT_FLAGS_WITH_VALUES.includes(arg[1])) {
				expected_flag = arg[1]
				continue
			}
			continue
		}
		if (arg.length < 3) {
			console.log(`What did you mean with ${arg} lol`)
			return 1
		}
		if (arg.length > 21) {
			console.log(`Flag name too long: ${arg}`)
			return 1
		}
		const flag = arg.slice(2).split('=', 1)
		const flag_name = flag[0]
		if (!FLAGS.includes(flag_name)) {
			console.log(`Unknown flag: --${flag_name}`)
			return 1
		}
		if (FLAGS_WITH_VALUES.includes(flag_name) && flag.length < 2) {
			console.log(`Missing flag value for --${flag_name}`)
			return 1
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
		console.log(`Missing flag value for -${expected_flag}`)
		return 1
	}
	/* if (flags[0].name === 'help') {
		return 0
	} */
	let code = ''
	if (from_cmd_line) {
		code = exec_flags['e']
	} else {
		if (i >= args.length) {
			console.log('Missing file path')
			return 1
		}
		const file_relative_path = args[i]
	}
	const exec_args = args.slice(i + 1)
	console.log({ executor_path, exec_flags, exec_args, code })
	return 0
}

function display_help() {
	const text = readFileSync('./help.txt').toString()
	console.log(text)
}

const FLAGS = [
	'help',
	'eval',
]

const SHORT_FLAGS = [
	'h',
	'e',
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

const exit_code = main(process.argv.slice(1))
process.exit(exit_code)