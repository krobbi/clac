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

Functions, closures, and built-in [standard library](standard_library.md)
functions are considered to have matching types and can be compared with each
other. Comparisons between different subtypes of functions will always produce
`false`:
```
clac> sqrt_wrapper(n) = sqrt(n), sqrt_wrapper == sqrt
false
```

## Relational Comparisons
Numbers can be compared as less than (`<`), less than or equal to (`<=`),
greater than (`>`), or greater than or equal to (`>=`) another number:
```
clac> 1 < 2
true

clac> 5 * 5 * 4 <= 100
true

clac> sqrt(2) < 1.5
true

clac> 0.1 + 0.2 >= 0.3
true
```

Only numbers can be used with relational comparisons:
```
clac> true > false
Error: type error
```

## Logical Operators
Logical operators take one or more Boolean values and apply logic to them.

The `!` operator produces the logical negation (NOT) of a Boolean value:
```
clac> !true, !false
false
true
```

The `&&` operator produces the logical and (AND) of two Boolean values. If both
values are `true`, then the result is `true`, otherwise it is `false`. If the
left-hand side value is `false`, then the right-hand side value will not be
evaluated:
```
clac> infinite_loop(f) = {f(f), 1}

clac> false && infinite_loop(infinite_loop) < 2
false
```

The `||` operator produces the logical or (OR) of two Boolean values. If either
value is `true`, then the result is `true`, otherwise it is `false`. If the
left-hand side value is `true`, then the right-hand side value will not be
evaluated:
```
clac> infinite_loop(f) = {f(f), 1}

clac> true || infinite_loop(infinite_loop) < 2
true
```

The `&&` and `||` operators can be chained:
```
clac> 1 > 2 || 3 > 4 || 5 < 6
true

clac> 7 < 8 && 9 < 10 && 11 > 12
false
```

Only Boolean values may be passed to logical operators:
```
clac> !0
Error: type error

clac> 1 && 2
Error: type error

clac> 3 || 4
Error: type error
```

## Conditional Expressions
Conditional logic can be implemented with a ternary conditional expression. A
Boolean condition is used before the `?` operator, followed by an expression to
evaluate if the condition is `true`. After this, the `:` operator is used,
followed by an expression to evaluate if the condition is `false`:
```
clac> absolute(n) = n < 0 ? -n : n

clac> absolute(123)
123

clac> absolute(-456)
456
```

Only Boolean values may be used as conditions:
```
clac> 1 ? 2 : 3
Error: type error
```

The conditional expression is short-circuited. Only the branch that was taken
will be evaluated.

The right-hand side of conditional expressions can be chained. For example,
`c1 ? b1 : c2 ? b2 : b3` is equivalent to:
```rust
if c1 {
    b1
} else if c2 {
    b2
} else {
    b3
}
```
