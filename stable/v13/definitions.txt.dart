// This is a Dart file because of the syntax highlighting

// a, b, c
CommaSep<T> = T ((',' | '\n') T)* ','?

// abc_123
Ident = ('a'..='z' | 'A'..='Z' | '_') ('a'..='z' | 'A'..='Z' | '_' | '0'..='9')*
// a::b::c.d.e
Ref = Ident ('::' Ident)* ('.' Ident)*
// a, [a, b], {x, y}, (i, j)
VarName = Ident | '[' CommaSep<VarName> ']' | '{' CommaSep<VarName> '}' | '(' CommaSep<VarName> ')'
// x(a, b: c)
Call = Ref PassedParams

// 123_456
Int = ('-' | '+')? '0'..='9' ('_'? '0'..='9')*
// 123.456
Float = Int ('.' Int)?
// 123.456e-789
Exp = Float ('e' | 'E') Float
// 0x123abc
Hex = '0x' ('0'..='9' | 'a'..='f' | 'A'..='F') ('0'..='9' | 'a'..='f' | 'A'..='F' | '_')*
// 0b0101_0101
Bin = '0b' ('0' | '1') ('0' | '1' | '_')*
Number = Int | Float | Exp | Hex | Bin

// 'a'
Char = 'b'? '\'' _ '\''

// $$"abc\n123"$$
SimpleString = ('$'*) '"' _* '"' ('$'*)
// r"abc"
RawString = 'r' SimpleString
// f"abc{123}"
FormatString = 'f' SimpleString
// c"abc"
CString = 'c' SimpleString
String = SimpleString | RawString | FormatString | CString

List = '[' CommaSep<Expr> ']'
Tuple = '(' CommaSep<Expr> ')'
// 1: "uno" | a.b.c
Entry = Expr ':' Expr | Ref
Construct = Ref (Tuple | '{' CommaSep<Entry> '}')
Value = Number | Char | String | Map | List | Tuple | Ref | Construct

Const = 'const' Ident ':' Typing '=' Value
State = 'state' Ident ':' Typing '=' Value

Generic = Ident (':' Typing)? ('=' Typing)?
Generics = ('<' CommaSep<Generic> '>')?
PassedGeneric = (Ident ':')? Typing
PassedGenerics = ('<' CommaSep<PassedGeneric> '>')?

FunType = 'async'? 'fun'? '(' CommaSep<Typing> ')' '->' Typing
SingleType = Ref Generics | FunType
// L[T; LEN], I + J
Typing = SingleType | SingleType? '[' SingleType (';' Expr)? ']' | SingleType '+' SingleType

Param = VarName (':' Typing)? ('=' Expr)?
Params = '(' CommaSep<Param> ')'
PassedParam = (Ident ':')? Expr
PassedParams = '(' CommaSep<PassedParam> ')'

Function = 'fun' Ident Generics Params ('->' Typing)? ('takes' '&'? 'self')? Block
UnsafeFunction = 'unsafe' Function
AsyncFunction = 'async' 'unsafe'? Function
StaticFunction = 'static' 'unsafe'? Function

SingleTypePattern = List | Tuple | Map | String | Number | Char | Ref | Construct
TypePattern = SingleTypePattern ('|' SingleTypePattern)*
MatchCase = TypePattern '=>' Expr
Match = 'match' Expr '{' CommaSep<MatchCase> '}'
MatchExpr = Expr 'is' TypePattern

Loop = 'loop' Block
While = 'while' Expr Block
DoWhile = 'do' While
For = 'for' VarName 'in' Expr Block
