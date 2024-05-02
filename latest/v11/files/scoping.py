
struct Scope {
  name: String,
  values: Map<String, Value>
}

let std_scope = {
  "println": Value::Constant {
    typed: Intermediate::BuiltIn(BuiltInType::Function),
    value: Intermediate::Function(Function { ... })
  }
}

let scopes: [Scope] = [
  Scope {
    name: "main.sny",
    values: {
      "TEST": Value::Constant {
        typed: Intermediate::Path(["i8"]),
        value: Intermediate::Number(Number::Int("5"))
      },
      "main": Value::Constant {
        typed: Intermediate::BuiltIn(BuiltInType::Function),
        value: Intermediate::Function(Function {
          name: "main",
          generics: [],
          args: [],
          body: [
            Expression::Call {
              name: Intermediate::Path(["println"]),
              generics: [],
              args: [
                Intermediate::Array([
                  Intermediate::Path(["TEST"]),
                  Intermediate::Path(["name"])
                ])
              ]
            }
          ]
        })
      }
    }
  },
  Scope {
    name: "main",
    values: {
      "name": Value::Variable Intermediate::String("Sunny")
    }
  }
]




