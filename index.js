import { readFileSync, existsSync } from 'node:fs'
import { exit, cwd } from 'node:process'
import { join } from 'node:path'
import { functions } from './functions.js'

function main(argv) {
	console.log(`[debug] argv [${argv.map(s => `"${s}"`).join(', ')}]`)
	const [, path, ...args] = argv
	if (!path) {
		console.log('Missing path')
		exit(1)
	}
	let file = ''
	let realPath = join(cwd(), path)
	if (existsSync(realPath)) {
		try {
			file = readFileSync(realPath).toString()
		} catch (error) {
			console.log(`InternalError: failed to read ${realPath}`)
			console.log({ error })
			exit(1)
		}
	} else {
		console.log(`[debug] ${realPath} not found`)
		if (!existsSync(realPath + '.sny')) {
			console.log(`File ${realPath} not found`)
			exit(1)
		}
		realPath += '.sny'
		try {
			file = readFileSync(realPath).toString()
		} catch (error) {
			console.log(`InternalError: failed to read ${realPath}`)
			console.log({ error })
			exit(1)
		}
	}
	try {
		console.log(`Executing ${realPath} with args [${args.join(', ')}]`)
		functions.eval(file, true)
	} catch (error) {
		console.log('Internal error')
		console.log({ error })
		exit(1)
	}
}



main(process.argv.slice(1))
// main(['sunny', 'main', 'some', 'args'])