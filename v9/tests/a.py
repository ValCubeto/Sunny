# this is not python, just syntax highlighting without errors everywhere

("{fun main() {\n  println('hello world')\n}}")
fun parse_mod(name, (ctx)) {
    assert(current_char == '{')
    next_char()
    while current_char != '}' {
        if char.is_alphanumeric() {
            word = collect_word()
            assert(word == "fun")
            match word {
                "struct" => {
                    skip_spaces()
                    name = expect_word()
                    assert(!global.has(name))
                    value = parse_struct(name)
                    global.add(name, value)
                }
                "enum" => {
                    skip_spaces()
                    name = expect_word()
                    assert(!global.has(name))
                    value = parse_enum(name)
                    global.add(name, value)
                }
                "class" => {
                    skip_spaces()
                    name = expect_word()
                    assert(!global.has(name))
                    value = parse_class(name)
                    global.add(name, value)
                }
                "fun" => {
                    skip_spaces()
                    name = expect_word()
                    assert(!global.has(name))
                    function = parse_function(name)
                    # panics if already declared
                    global.add(name, function)
                }
                "mod" => {
                    skip_spaces()
                    name = expect_word()
                    assert(!global.has(name))
                    mod = parse_mod(name)
                    global.set(name, mod)
                }
                "const" => {
                    skip_spaces()
                    name = expect_word()
                    assert(!global.has(name))
                    skip_spaces()
                    expect_char(':')
                    skip_spaces()
                    type = expect_type()
                    skip_spaces()
                    expect_char('=')
                    skip_spaces()
                    value = expect_value()
                    global.set(name, Constant { type, value })
                }
                "use" => {
                    skip_spaces()
                    # you can import at the same time or in separate statements
                    # so i add them all
                    paths = parse_paths()
                    imports.add(paths)
                }
            }
        }
    }
}

fun import_all() {
    # do not keep the entire module, unload it after imported the needed values
    for path in imports {
        # change the method if its from std or an external module
        module: Value = load(path).expect("failed to load {path}")
        for (k, v) in module {
            # verificar antes que no haya nombres que colapsen
            global.set(k, v)
        }
    }
}