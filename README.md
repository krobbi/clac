# Clac
Clac was originally created as a command line calculator to address some
annoyances with the default Windows calculator:
* Relatively long time to start up
* No support for custom functions and variables
* No way to evaluate an expression from a command line session

# Usage
Clac is run from the command line:
```shell
clac [CODE]
```

If one or more arguments are given, then they are joined with spaces and
treated as a single line of code. Clac executes the code and exits
automatically.

If no arguments are given, then the user can enter code in a loop until
manually exiting with `Ctrl+D` (Linux, macOS, etc.) or `Ctrl+Z` (Windows.)

# Language
The decision to support functions and variables in Clac has expanded its scope
to being a small, mathematical (but not yet Turing-complete) scripting
language.

Clac is designed to be usable as a calculator, so writing an expression at the
top level of a program will print its result:
```
clac> 1 + 1
2
```

A program may contain zero or more expressions, separated by commas. A trailing
comma may also be used:
```
clac> 1.5 + 2 * 3, (1.5 + 2) * 3,
7.5
10.5
```

Commas are optional, but are sometimes necessary to separate expressions:
```
clac> 0 1 -2
0
-1

clac> 0 1, -2
0
1
-2
```

## Variables
Variables can be defined with the `=` operator:
```
clac> x = 5, x * x
25

clac> x
5
```

Reassigning an existing variable is not currently supported:
```
clac> count = 1, count = 1 + 1
Error: variable 'count' is already defined
```

Variable names must consist of one or more ASCII letters or underscores, with
digits allowed after the first character.

### Expressions and Statements
Variable definitions are not printed because they are statements, not
expressions. Unlike expressions, statements do not produce a value. This means
that variable definitions cannot be chained:
```
clac> x = y = 1
Error: assignments cannot be chained
```

<!--
## Blocks
Zero or more expressions can be grouped into a block by surrounding them with
curly braces and separating them with commas:
```
clac> {}

clac> {1}
1

clac> {1, 2, 3}
3
```

The expressions in the block are evaluated in order and the value returned by
the last expression is returned by the block. If the block is empty or the last
expression does not return a value, then the block will not return a value.

### Scopes
Beginning a block creates a new innermost scope. Variables defined inside a
block cannot be used after the block:
```
clac> global = 5, global, {local = 2 * global, local}, local
5
10
Runtime error: variable 'local' is undefined
```

When a variable is assigned, an already defined variable in the innermost scope
is attempted to be assigned, following the next scope outwards until the global
scope is reached. A new variable will only be defined in the innermost scope if
no variable with the same name was found in any outer scope:
```
clac> shadow = 0, {shadow = 1}, shadow
1
```

## Functions
Functions can be called by following a function expression with zero or more
arguments surrounded by parentheses and separated by commas:
```
clac> sqrt(25)
5
```

Functions are values that can be stored in variables, and passed to and
returned from functions:
```
clac> magic = sqrt

clac> get_magic() = magic

clac> apply(f, x) = f(x)

clac> apply(get_magic(), 64)
8
```

### User-defined Functions
Functions can be defined with algebraic syntax by assigning a body expression
to a function 'call' that defines the function's name and parameters:
```
clac> f(x) = 2 * x + 1

clac> f(1), f(2), f(3)
3
5
7
```

The function's name and parameters must be defined as identifiers
(variable names.) Additionally, each parameter must have a different name to
any other parameters:
```
clac> 1(x) = {}
Runtime error: function names must be identifiers

clac> f(1) = {}
Runtime error: function parameter names must be identifiers

clac> f(x, x) = {}
Runtime error: functions cannot have duplicate parameter names
```

### Function Scoping
Unlike variable assignments, user-defined functions are always defined as a new
variable in the innermost scope:
```
clac> shadow = 0, {shadow() = 1, shadow}, shadow
function
0
```

Function calls always create a new innermost scope for their parameters, even
if the function body is not a block:
```
clac> f(x) = result = 2 * x

clac> f(10)

clac> result
Runtime error: variable 'result' is undefined
```

Functions are dynamically scoped, meaning they have access to the variables
that were in scope *when the function was called*. This makes the language
easier to implement, but can cause confusing or unexpected behavior:
```
clac> triple_value() = 3 * value

clac> {value = 123, triple_value()}
369

clac> triple_value()
Runtime error: variable 'value' is undefined
```

Unfortunately, this means it is not possible to create closures with the
current function scoping rules:
```
clac> new_counter() = {n = 0, counter() = {n = n + 1, n}, counter}

clac> counter = new_counter()

clac> counter()
Runtime error: variable 'n' is undefined
```

### Built-in Functions
The Clac language includes built-in functions for commonly-used operations:
| Function                    | Usage                           |
| :-------------------------- | :------------------------------ |
| `sqrt(n: number) -> number` | Returns the square root of `n`. |
-->

## Grammar
All valid Clac programs should have the following grammar:
```ebnf
program  = sequence, Eof ;
sequence = { stmt, [ "," ] } ;
stmt     = expr, [ "=", expr ] ;
tuple    = "(", [ expr, { ",", expr }, [ "," ] ], ")" ;
expr     = expr_sum ;

expr_sum     = expr_term, { ( "+" | "-" ), expr_term } ;
expr_term    = expr_prefix, { ( "*" | "/" ), expr_prefix } ;
expr_prefix  = "-", expr_prefix | expr_call ;
expr_call    = expr_primary, { tuple } ;
expr_primary = "(", expr, ")" | "{", sequence, "}" | Number | Ident ;
```

# Credits
* Infix parser based on
[pseudocode by Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing#precedence-climbing-how-it-actually-works).

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
