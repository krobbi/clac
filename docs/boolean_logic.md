[Go back](README.md)

# Boolean Logic
Clac supports Boolean values (true and false), however there is not yet any
comparison or conditional logic to use them with.

The `true` and `false` keywords are used to represent Boolean values:
```
clac> true, false
true
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
