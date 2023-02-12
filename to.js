import { id } from './identifiers.js'

export const to = {
	'bool': function (value) {
		return Value(id.boolean, value ? 1 : 0)
	},
	/**
	 * @typedef {{ type: 'string', value: string }} str
	 * @param {string} value 
	 * @returns {str}
	 */
	'str': function (value) {
		if (typeof value === 'object' && id.type in value) {
			switch (value[id.type]) {
				case id.void:
					return Value(id.string, 'null')
				case id.boolean:
					return Value(id.string, value[id.value] === 1 ? 'true' : 'false')
				case id.number:
					return Value(id.string, value[id.value])
				case id.range:
					return Value(id.string, `<range between ${value[id.value][0]} and ${value[id.value][1]}>`)
				case id.string:
					return value
				case id.regexp:
					return Value(id.string, `/${value[id.value]}/${value[id.data][id.flags]}`)
				case id.array:
					return Value(id.string, `<array with ${value[id.value].length} elements>`)
				case id.object:
					return Value(id.string, id.xtype in value ? `<${value[id.xtype][id.name]} object>` : '<object>')
				case id.function:
					return Value(id.string, `<function ${value[id.data][id.name]}>`)
			}
		}
		return Value(id.string, `${value}`)
	},
	'arr': function (value) {
		return Value(id.array, value)
	},
	'obj': function (value, xtype) {
		return Value(id.object, value, xtype ? { [id.xtype]: xtype } : null)
	},
	'Function': function (value, data) {
		return Value(id.function, value, data)
	},
	'param': function (name, type, additional) {
		const param = {
			[id.name]: name,
			...additional
		}
		if (type) param[id.type] = type
		return param
	}
}

function Value(type, value, data) {
	const object = {
		[id.type]: type,
		[id.value]: value
	}
	if (data) {
		object[id.data] = data
	}
	return object
}