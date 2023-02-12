/**
 * @param {string} key
 * @returns {string}
*/
function id_for(key) {
	return key
	if (!ids.includes(key)) {
		return (ids.push(key) - 1).toString()
	}
	return (ids.indexOf(key)).toString()
}

const ids = []

export const id = {
	/*           root            */
	'type'     : id_for(  'type'  ),
	'data'     : id_for(  'data'  ),
	'value'    : id_for( 'value'  ),
	/*        root type          */
	'declare'  : id_for('declare' ),
	'assign'   : id_for( 'assign' ),
	'call'     : id_for(  'call'  ),
	'condition': id_for('condition'),
	'if': id_for('if'),
	'else': id_for('else'),
	'then': id_for('then'),
	/*          token            */
	'begin'    : id_for( 'begin'  ),
	'end'      : id_for(  'end'   ),
	/*        token type         */
	'word'     : id_for(  'word'  ),
	'symbol'   : id_for( 'symbol' ),
	'eol'      : id_for(  'eol'   ),
	'indent'   : id_for( 'indent' ),
	/*        value data         */
	'xtype'    : id_for( 'xtype'  ),
	'match'    : id_for( 'match'  ),
	'constant' : id_for('constant'),
	'name'     : id_for(  'name'  ),
	'list'     : id_for(  'list'  ),
	'internal' : id_for('internal'),
	/*        value type         */
	'void'     : id_for(  'void'  ),
	'boolean'  : id_for('boolean' ),
	'number'   : id_for( 'number' ),
	'string'   : id_for( 'string' ),
	'array'    : id_for( 'array'  ),
	'object'   : id_for( 'object' ),
	'function' : id_for('function'),
	'class'    : id_for( 'class'  ),
	/*        params/args        */
	'params'   : id_for( 'params' ),
	'args'     : id_for(  'args'  ),
	'rest'     : id_for(  'rest'  ),
	'optional' : id_for('optional'),
	'default'  : id_for('default' ),
}