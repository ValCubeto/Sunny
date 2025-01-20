// This is a Dart file because of the syntax highlighting

// a, b, c
CommaSep<T> = T ((',' | '\n') T)* ','?

// abc_123
Ident = ('a'..='z' | 'A'..='Z' | '_') ('a'..='z' | 'A'..='Z' | '_' | '0'..='9')*
// a::b::c.d.e
Ref = Ident ('::' Ident)* ('.' Ident)*
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
Raw_String = 'r' SimpleString
// f"abc{123}"
Format_String = 'f' SimpleString
// c"abc"
CString = 'c' SimpleString
String = SimpleString | Raw_String | Format_String | CString

List = '[' CommaSep<Expr> ']'
Tuple = '(' CommaSep<Expr> ')'
// 1: "uno" | a.b.c
Entry = Expr ':' Expr | Ref
Map = '{{' CommaSep<Entry> '}}'
Construct = Ref ('{' CommaSep<Entry> '}' | Tuple | Map)

Value = Number | Char | String | Map | List | Tuple | Ref | Construct

Generic = Ident (':' Typing)? ('=' Typing)?
Generics = ('<' CommaSep<Generic> '>')?
PassedGeneric = (Ident ':')? Typing
PassedGenerics = ('<' CommaSep<PassedGeneric> '>')?

Param = Ident (':' Typing)? ('=' Expr)?
Params = '(' CommaSep<Param> ')'
PassedParam = (Ident ':')? Expr
PassedParams = '(' CommaSep<PassedParam> ')'

FunType = 'fun'? Params '->' Type
Type = Ref Generics | FunType
Typing = SingleType | SingleType '[' (Int | Ref)? ']' | Type '+' Type

Const = 'const' Ident ':' Typing '=' Value
State = 'state' Ident ':' Typing '=' Value

Function = 'fun' Ident Generics Params ('->' Type)? ('takes' '&'? Ident)? Block
UnsafeFunction = 'unsafe' Function
AsyncFunction = 'async' 'unsafe'? Function
StaticFunction = 'static' 'unsafe'? Function

SingleTypePattern = List | Tuple | Map | String | Number | Char | Ref | Construct
TypePattern = SingleTypePattern ('|' SingleTypePattern)*
MatchCase = TypePattern '=>' Expr
Match = 'match' Expr '{' CommaSep<MatchCase> '}'
MatchExpr = Expr 'is' TypePattern
