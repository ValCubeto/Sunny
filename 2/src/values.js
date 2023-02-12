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
	'number': function (value, is_internal) {
		return {
			type: 'number',
			value,
			is_internal
		}
	},
	'range': function (a, b) {},
	'string': function (value) {},
	'regexp': function (value) {},
	'array': function (value) {
		return {
			type: 'array'
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
	'function': function (name, params, returns, value) {
		const f = {
			name: name ?? 'unnamed',
			value,
			is_internal: typeof value === 'function'
		}
		if (params) {
			f.params = params
		}
		if (returns) {
			f.returns = returns
		}
		return f
	}
}

const UNSET = create.void(0)
const NULL = create.void(1)

const FALSE = create.boolean(0)
const TRUE = create.boolean(1)

const NEG_INFY = create.number(0, true)
const NANUM = create.number(1, true)
const INFY = create.number(2, true)

export {
	create,
	UNSET, NULL,
	FALSE, TRUE,
	NEG_INFY, NANUM, INFY
}