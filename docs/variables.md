[Go back](README.md)

# Variables
Variables are defined with the `=` operator, with a variable name on the
left-hand side of the operator and a value on the right:
```
clac> x = 5, x * x
25

clac> x
5
```

## Variable Names
Variable names must consist of one or more ASCII letters or underscores. After
the first character, digits are also allowed in variable names.

Plain variable names must be used in variable definitions. They cannot be
surrounded by parentheses:
```
clac> (x) = 20
Error: can only assign to variables and function signatures
```

## Mutability
Currently, all variables are immutable and cannot be reassigned:
```
clac> count = 1, count = 1 + 1
Error: variable 'count' is already defined
```

## Variable Definitions are Statements
Variable definitions are statements, not expressions. This separates the effect
of defining a variable from the evaluation of expressions.

Stored values will not be printed when variable definitions are used at the top
level of a program. Variable definitions also cannot be chained:
```
clac> x = y = 1
Error: assignments cannot be chained
```
