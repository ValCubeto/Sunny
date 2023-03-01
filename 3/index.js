/**
 * @param {string[]} args 
 */
function main(args) {
	const executor_path = args[0]
	const flags = {}
	let expected_flag = ''
	let i = 0
	for (i = 1; i < args.length; i++) {
		const arg = args[i]
		if (expected_flag !== '') {
			flags[expected_flag] = arg
			expected_flag = ''
			continue
		}
		if (arg[0] !== '-' || arg.length <= 1) {
			break
		}
		if (arg[1] !== '-') {
			expected_flag = arg.slice(1)
			continue
		}
		if (arg.length === 2) {
			break
		}
		const [flag_name, flag_value = ''] = arg.slice(2).split('=')
		flags[flag_name] = flag_value
	}
	const file_relative_path = args[i]
	const exec_args = args.slice(i + 1)
	console.log({ executor_path, flags, file_relative_path, exec_args })
}

const exit_code = main(process.argv.slice(1))
process.exit(exit_code)