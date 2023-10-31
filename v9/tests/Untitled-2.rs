priors: [u8; _] = [
  [And(x, y), Or(x, y)]
  [Eq(x, y), Neq(x, y)]
  [Lt(x, y), Gt(x, y), LtEq(x, y), GtEq(x, y)]
  [Mod(x, y)]
  [Pow(x, y)]
  [Mul(x, y), Div(x, y)],
  [Add(x, y), Sub(x, y)]
  [Not(x), Pos(x), Neg(x)]
  [Value]
]

"5 < (1 + 2) * 3"
Lt(
  Value(5),
  Sum(
    Value(1),
    Mul(
      Value(2),
      Value(3)
    )
  )
)
Lt(
  Value(5),
  Mul(
    Sum(
      Value(1),
      Value(2),
    ),
    Value(3)
  )
)

Lt(
  Value(5),
  Sum()
)