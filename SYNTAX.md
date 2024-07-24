uhh silly language

elements are defined with usually just one letter
then are styled using the following characters
`↑` => bottom:
`↓` => top:
`→` => left:
`←` => right:
`↔` => width
`↕` => height


d↔200↕100\⇡⍳10

value? monop? operator value
here the ? represents a maybe value

expr = (monop)? prefixop expr | prim ((monop)? binop expr)?;
prim = "(" expr ")" | literal;
literal = "string" | number | [ literal ]*
