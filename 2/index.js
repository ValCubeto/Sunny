import { existsSync, lstatSync, readFileSync } from 'node:fs'
import { join as join_paths } from 'node:path'
import { key_throw } from './src/keys.js'
import { EXTENSION } from './src/info.js'

function main(argv) {
	console.log(`[debug] Executing with argv = ${JSON.stringify(argv, null, 2)}`)
	const path = argv[1]
	console.log(`[debug] Looking for main file`)
	const file = try_read([path, path + EXTENSION])
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