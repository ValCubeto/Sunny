import { existsSync, lstatSync, readFileSync } from 'node:fs'
import { join as join_paths } from 'node:path'
import { key_throw } from './src/keys.js'
import { EXTENSION } from './src/info.js'
import { eval_code } from './src/functions.js'

function main(argv) {
	// Example: sunny --allow-read main some args
	console.log(`[debug] Executing with argv = ${JSON.stringify(argv, null, 2)}`)

	// Parse argv
	const executor_path = argv[0]
	const flags = []
	let i = 1
	for (; i < argv.length; i++) {
		if (argv[i][0] !== '-') {
			break
		}
		flags.push(argv[i])
	}
	const path = argv[i]
	const args = argv.slice(i + 1)
	console.log(`[debug] ${JSON.stringify({ executor_path, flags, path, args }, null, 2)}`)

	// Read file
	console.log(`[debug] Looking for main file`)
	const file = try_read([path, path + EXTENSION])

	if (file.length === 0) {
		key_throw('The file is empty')
	}

	eval_code(file)
}

const WORKING_DIR = process.cwd()

/**
 * @param {string[]} paths
 * @returns {string}
 */
function try_read(paths) {
	for (const i in paths) {
		const path = join_paths(WORKING_DIR, paths[i])
		if (existsSync(path)) {
			if (!lstatSync(path).isFile()) {
				key_throw(`"${path}" is not a file`)
			}
			try {
				console.log(`[debug] Reading "${path}"`)
				const file = readFileSync(path).toString()
				console.log(`[debug] Working with "${path}"`)
				return file
			} catch (error) {
				key_throw(`Failed to read "${path}" (code: ${error.code})`)
			}
		}
		console.log(`[debug] File at "${path}" not found`)
	}
	key_throw(`File not found`)
}

main(process.argv.slice(1))