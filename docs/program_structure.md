[Go back](README.md)

# Program Structure
A Clac program is made up of zero or more expressions and statements, separated
by commas. A trailing comma may also be used:
```
clac> 1.5 + 2 * 3, (1.5 + 2) * 3,
7.5
10.5
```

Commas are optional, but sometimes necessary as separators:
```
clac> 0 1 -2
0
-1

clac> 0 1, -2
0
1
-2
```

## Expressions
An expression is anything that produces a value. For example, `3.14` is an
expression. `2 * 3` and `-(4 + 5) / 6` are also expressions and are made up of
sub-expressions.

When an expression is used at the top level of a program, its resulting value
is printed. This enables the language to be used as a calculator:
```
clac> 0.77 + 0.0034
0.7734
```

## Statements
Statements are similar to expressions, but do not produce a value. For example,
[variable](variables.md) definitions (`x = 123`) and empty [blocks](blocks.md)
(`{}`) are statements.

Because statements do not produce a value, they do not print anything when used
at the top level of a program:
```
clac> x = 123

clac> x
123
```

Statements cannot be used as a sub-expression:
```
clac> 1 + {}
Error: statements cannot be used as operands
```
