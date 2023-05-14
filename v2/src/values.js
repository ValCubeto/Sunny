import { key_equals } from './keys.js'

const create = {
	'void': function (value) {
		return {
			type: 'void',
			value
		}
	},
	'boolean': function (value) {
		return {
			type: 'boolean',
			value
		}
	},
	'number': function (value, internal) {
		if (!internal && 'number' in cache && value in cache.number) {
			return cache.number[value]
		}
		const n = {
			type: 'number',
			value
		}
		if (internal) {
			n.data = { internal: true }
		}
		cache.number ??= {}
		cache.number[value] = n
		return n
	},
	'range': function (x, y) {
		const r = {
			type: 'range'
		}
		r.value = x <= y ? [x, y] : [y, x]
		return r
	},
	'string': function (value) {
		return {
			type: 'string',
			value
		}
	},
	'regexp': function (body, flags) {
		const rx = {
			type: 'regexp',
			value: body
		}
		if (flags) {
			rx.data = { flags }
		}
		return rx
	},
	'array': function (value) {
		return {
			type: 'array',
			value
		}
	},
	'object': function (entries) {
		const keys = new Array(entries.length)
		const values = new Array(entries.length)
		for (const i in entries) {
			const [key, value] = entries[i]
			keys[i] = key
			values[i] = value
		}
		return {
			type: 'object',
			value: [keys, values]
		}
	},
	'type': function (name, value, model) {
		const t = {
			type: 'type',
			data: {
				name
			},
			value
		}
		if (model) {
			t.data.model = model
		}
		return t
	},
	'model': function (value) {
		return {
			type: 'model',
			value
		}
	},
	'function': function ({ name, params, returns, value }) {
		const f = {
			data: {
				name: name ?? 'unnamed'
			},
			value
		}
		if (params) {
			f.data.params = params
		}
		if (returns) {
			f.data.returns = returns
		}
		if (typeof value === 'function') {
			f.data.internal = true
		}
		return f
	}
}

const cache = {}

const UNSET = create.void(0)
const NULL = create.void(1)

const FALSE = create.boolean(0)
const TRUE = create.boolean(1)

const NEG_INFY = create.number(0, true)
const NANUM = create.number(1, true)
const INFY = create.number(2, true)

function test_type(type, value_to_test) {
	if (type.value.type === 'type.in_list') {
		for (const value_to_compare of type.value.value) {
			if (key_equals(value_to_compare, value_to_test)) {
				return true
			}
		}
		return false
	}
}

function to_string(value, indent = '') {
	if (value.type === 'string') {
		return create.string(`'${value.value}'`)
	}
	if (value.type === 'void') {
		return create.string('null')
	}
	if (value.type === 'boolean') {
		return create.string(value.value ? 'true' : 'false')
	}
	if (value.type === 'number') {
		if (value.data?.internal) {
			return create.string(['-infy', 'nanum', 'infy'][value.value])
		}
		return create.string(value.value)
	}
	if (value.type === 'range') {
		return create.string(`Range(${value.value[0]}, ${value.value[1]})`)
	}
	if (value.type === 'regexp') {
		return create.string(`/${value.value}/${value.data.flags}`)
	}
	if (value.type === 'array') {
		let string = '[\n'
		for (const i in value.value) {
			if (value.value[i] === value) {
				string += `${indent}  <circular>`
			}
			string += `${indent}  ${to_string(value.value[i], indent + '  ')}`
		}
		string += '\n]'
		return create.string(string)
	}
	if (value.type === 'object') {
		let string = '{'
		for (const i in value.value[0]) {
			if (value.value[1][i] === value) {
				string += `${indent}  <circular>`
			}
			string += `\n${indent}  ${value.value[0][i]} = ${to_string(value.value[1][i], indent + '  ').value}`
		}
		string += '\n}'
		return create.string(string)
	}
	if (value.type === 'function') {
		return create.string(`fun ${value.data.name}()`)
	}
	return create.string('unknown')
}

function obj_get_value(object, key) {
	const index = object.value[0].indexOf(key)
	return object.value[1][index]
}
function obj_add_value(object, key, value) {
	object.value[0].push(key)
	object.value[1].push(value)
}
function obj_set_value(object, key, value) {
	const index = object.value[0].indexOf(key)
	if (index === -1) {
		obj_add_value(object, key, value)
	}
	object.value[0][index] = key
	object.value[1][index] = value
}
function obj_get_entries(object, raw) {
	const entries = []
	for (const i in object.value[0]) {
		const key = object.value[0][i]
		const value = object.value[1][i]
		entries.push(raw ? [key, value] : create.array([key, value]))
	}
	return raw ? entries : create.array(entries)
}

const g = create.object([
	// values
	['null',  UNSET],
	['false', FALSE],
	['true',  TRUE],
	['nanum', NANUM],
	['infy',  INFY],
	// types
	['void', create.type('void', { type: 'type.in_list', value: [UNSET, NULL] })],
])

const CONSTANT = { constant: true }

const data = {}

// make all global values constants
for (const i in g.value[0]) {
	data[g.value[0][i]] = CONSTANT
}

obj_add_value(g, 'global', g)

export {
	create,
	UNSET, NULL,
	FALSE, TRUE,
	NEG_INFY, NANUM, INFY,
	obj_get_value, obj_add_value, obj_set_value, obj_get_entries,
	to_string,
	g
}