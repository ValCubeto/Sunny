# this is not python, just syntax highlighting without errors everywhere
# comment

##
 # comment
##

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








Class::new("u8", empty_struct, hashset! [], hashmap! {
    "to_string" => Constant {
        public: true,
        typeof: FunctionPtr,
        value: builtin!(|args| { asd })
    }
})

struct Struct {
    name: Id,
    props: HashSet<(Id, StructPtr)> # (name, type)
}

struct Instance {
    typeof: StructPtr,
    props: HashSet<Value>
}

struct Enum<'a> {
    name: Id,
    variants: &'a [StructPtr]
}

struct Class {
    inner_struct: StructPtr,
    public: HashMap<Id, Value>
}

enum Value {
    String(Id),
    u8(u8),
    Function(Function),
    Mod, Class, Struct
    ...
}

struct Variable {
    public: bool,
    mutable: bool,
    typeof: Rc<Struct>,
    value: Value
}
struct Constant {
    public: bool
    typeof: Rc<Struct>,
    value: Value
}













struct Mod {
    name: Id,
    values: HashMap<Id, Variable>
}

let mod = Mod::new("main", hashmap! {
    "NAME" => inmutable! {
        Rc::clone(&structs["String"]),
        Value::String("Sunny")
    }
})

mod.values.get("NAME")