export function exec_ast(ast, stack = [], expression, internal, main) {
	for (const { type, data, value } of ast) {
		switch (type) {
			case 'get':
				if (!expression) {
					console.log(`Warning: valor suelto '${data.name}'`)
				}
		}
	}
}