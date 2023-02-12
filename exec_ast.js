import { id } from './identifiers.js'
import { functions } from './functions.js'
import { to } from './to.js'
import { keys } from './keys.js'

export function exec_ast(ast, stack = dataStack) {
	if (!Array.isArray(stack)) {
		keys.throw('InternalError: invalid data stack', { stack })
	}
	const unused = []
	for (const root of ast) {
		const value = root[id.data]
		switch (root[id.type]) {
			case id.declare:
				console.log(`[debug] declare ${value[id.name]}`)
				if (searchValue(stack, value[id.name]) !== -1) {
					keys.throw(`ReferenceError: '${value[id.name]}' is already defined`)
				}
				stack.at(-1)[value[id.name]] = root[id.value]
				unused.push(value[id.name])
				break
			case id.assign:
				console.log(`[debug] assign ${value[id.name]}`)
				if (searchValue(value[id.name], stack[id.value])) {
					if (constant in globalData[id.value][value[id.name]][id.data]) {
						const route = id.list in value
							? value[id.list].concat(value[id.name]).join('.')
							: value[id.name]
						keys.throw(`TypeError: '${route}' is a constant`)
					}
					if (!type_equals(globalData[id.value][root[id.name]][id.type], root[id.value][id.type])) {
						const type_1 = repr_type(root[id.value][id.type])
						const type_2 = repr_type(globalData[id.value][root[id.name]][id.type])
						keys.throw(`TypeError: cannot assign '${type_1}' to '${type_2}'`)
					}
				}
				globalData[id.value][value[id.name]] = root[id.value]
				break
			case id.call:
				if (unused.includes(value[id.name])) {
					// remove the element bc was used
					unused.splice(unused.indexOf(value[id.name]), 1)
				}
				const f = getValue(stack, value[id.name])
				const args = make_args(f, value[id.params])
				exec_ast(f[id.value], stack.concat(args))
				break
			default:
				keys.throw(`[debug] ignored root '${root[id.type]}'`)
		}
		// ignore if global scope
		if (/* stack.length > 1 &&  */unused.length) {
			for (const name of unused) {
				console.log(`Warning: se declaró '${name}', pero nunca se usó`)
			}
		}
	}
}

function make_args({ [id.params]: params = [] }, argList = []) {
	// {[id.name] = 'test', [id.value] = str('Hello')}[]
	// { test = str('Hello') }
	if (!params.length) return {}
	const args = {}
	let checkedArgs/*: number[] */ = []
	for (const i in argList) {
		const arg = argList[i]
		if (id.name in arg) {
			const index = params.findIndex(param => param[id.name] === arg[id.name])
		}
	}
	return args
}

function searchValue(stack, name) {
	console.log(`[debug] search ${name}`)
	for (let i = stack.length - 1; i >= 0; i--) {
		if (name in stack[i]) {
			console.log(`[debug] ${name} found at stack[${i}]`)
			return i
		}
		console.log(`[debug] ${name} not in stack[${i}]`)
	}
	return -1
}

function getValue(stack, name) {
	console.log(`[debug] get ${name}`)
	const stackIndex = searchValue(stack, name)
	if (stackIndex === -1) {
		keys.throw(`ReferenceError: '${name}' is not defined`)
	}
	return stack[stackIndex][name]
}

const VOID = {
	[id.type]: id.void,
	[id.data]: {
		[id.constant]: true
	},
	[id.value]: 0
}

const NULL = {
	[id.type]: id.void,
	[id.data]: {
		[id.constant]: true
	},
	[id.value]: 1
}

/**
 * @type {{ type: string, value: Record<string, { type: string, value: any, data?: Record<string, unknown> }> }}
 */
const globalData = to.obj({
	'null': NULL,
	'false': {
		[id.type]: id.boolean,
		[id.data]: {
			[id.constant]: true
		},
		[id.value]: 0
	},
	'true': {
		[id.type]: id.boolean,
		[id.data]: {
			[id.constant]: true
		},
		[id.value]: 1
	},
	'nanum': {
		[id.type]: id.number,
		[id.data]: {
			[id.constant]: true,
			[id.internal]: true
		},
		[id.value]: 0
	},
	'infy': {
		[id.type]: id.number,
		[id.data]: {
			[id.constant]: true,
			[id.internal]: true
		},
		[id.value]: 1
	},
	'print': to.Function(
		function ({ values, sep, end }) {
			functions.print(values, sep, end)
		},
		{
			[id.internal]: true,
			[id.params]: [
				to.param('values', {
					[id.rest]: true,
					[id.xtype]: {
						[id.name]: 'arr'
					}
				}),
				to.param('sep', {
					[id.optional]: true,
					[id.xtype]: {
						[id.name]: 'str'
					}
				}),
				to.param('end', {
					[id.optional]: true,
					[id.xtype]: {
						[id.name]: 'str'
					}
				}),
			]
		}
	),
	'exists': to.Function(
		function ({ name }) {
			return to.bool(searchValue(name) !== -1)
		},
		{
			[id.params]: [
				{
					[id.name]: 'name',
					[id.type]: id.string
				}
			]
		}
	)
})
// globalData[value]['global'] = globalData

const dataStack = [globalData[id.value]]

function repr_type(type) {
	/* if (typeof type === 'string') {
		return `[${ids[type]}]`
	} */
	if (id.model in type) {
		const model_repr = Object.entries(type[id.model]).map(([key, value]) => `${key} = ${value}`).join(', ')
		return `${type[id.name]}<${model_repr}>`
	}
	return type[id.name]
}