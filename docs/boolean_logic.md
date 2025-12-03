[Go back](README.md)

# Boolean Logic
The `true` and `false` keywords are used to represent Boolean values:
```
clac> true, false
true
false
```

## Equality Comparisons
Two values can be compared as equal with the `==` operator, or as not equal
with the `!=` operator:
```
clac> 1 + 2 == 3, 9 + 10 != 21
true
true
```

Comparisons must use values with matching types:
```
clac> true == 1
Error: type error
```

Exact comparisons between numbers with decimal places may be inaccurate because
Clac uses floating point numbers:
```
clac> 0.1 + 0.2 == 0.3
false

clac> 0.1 + 0.2
0.30000000000000004
```

[Functions](functions.md) are equal if they refer to the same definition in
source code. Functions defined in different places are never considered equal,
even if they have the exact same code:
```
clac> foo() = 1, bar() = 1, baz = foo

clac> foo == bar
false

clac> foo == baz
true

clac> (() -> 123) == (() -> 123)
false

clac> get_anon() = () -> 123, get_anon() == get_anon()
true
```

Closures are equal if they are capturing equal values with equal functions:
```
clac> new_adder(c) = n -> n + c, adder = new_adder(1)

clac> adder == new_adder(1)
true

clac> adder == new_adder(2)
false

clac> adder == (c -> n -> n + c)(1)
false
```

Functions, closures, and [standard library](standard_library.md) functions are
considered to have matching types and can be compared with each other.
Comparisons between different subtypes of functions will always produce
`false`:
```
clac> sqrt_wrapper(n) = sqrt(n), sqrt_wrapper == sqrt
false
```

## Logical Operators
Logical operators take one or more Boolean values and apply logic to them.

The `!` operator produces the logical negation (NOT) of a Boolean value:
```
clac> !true, !false
false
true
```

Only Boolean values may be passed to logical operators:
```
clac> !0
Error: type error
```
