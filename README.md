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

Variables cannot be defined if the variable name is surrounded by parentheses:
```
clac> (x) = 20
Error: can only assign to variables and function signatures
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

## Blocks
Zero or more expressions and statements can be grouped into a block by
surrounding them with curly braces. The items in a block can be separated with
commas using the same rules as the top level of the program. If a block ends
with an expression, then the block is an expression that produces the value of
its last expression:
```
clac> {1, 2, 3}
3
```

If a block is empty or ends with a statement, then the block is a statement
that does not produce a value:
```
clac> 123, {} {4, x = 5} {6, {}} 789
123
789
```

### Block Scopes
Each block has its own scope. Variables defined inside a block can be used in
the block and any nested blocks, but cannot be used after the block ends:
```
clac> global = 5, {local = 2 * global, {local}}
10

clac> global
5

clac> local
Error: variable 'local' is undefined

clac> {{inner = 100} inner}
Error: variable 'inner' is undefined
```

Block scopes allow variables to be defined with names that are already defined
in an outer scope. The new variable temporarily 'shadows' the old variable and
does not modify it:
```
clac> depth = 0, {depth = depth + 1, depth} depth
1
0
```

## Functions
Functions can be called by following the function with zero or more arguments
surrounded by parentheses:
```
clac> sqrt(25)
5
```

Functions must be called with the expected number of arguments:
```
clac> sqrt()
Error: incorrect number of arguments for function call

clac> sqrt(1, 2)
Error: incorrect number of arguments for function call
```

### User-defined Functions
Functions can be defined with algebraic syntax. An expression defining the
function's body can be assigned to a 'call' that defines the function's name
and parameters:
```
clac> f(x) = 2 * x + 1

clac> f(1), f(2), f(3)
3
5
7
```

Function bodies must be an expression that produces a value:
```
clac> nop() = {}
Error: functions must return a value
```

Unlike blocks and the top level of the program, function parameters and call
arguments *must* be separated with commas. Trailing commas may optionally be
used:
```
clac> sqdist(x0 x1) = {dx = x1 - x0, dx * dx}
Error: expected a closing ')', got identifier 'x1'

clac> sqdist(x0, x1,) = {dx = x1 - x0, dx * dx}

clac> sqdist(10 20)
Error: expected a closing ')', got number '20'

clac> sqdist(10, 20,)
100
```

The function's name and parameters must be defined as identifiers
(variable names) that are not surrounded by parentheses. Each parameter must
also have a unique name:
```
clac> 1(x) = x + 1
Error: function names must be identifiers

clac> (f)(x) = x + x
Error: function names must be identifiers

clac> f(1) = 2
Error: function parameters must be identifiers

clac> f((x)) = 1 / x
Error: function parameters must be identifiers

clac> f(x, x) = x * x
Error: function parameter 'x' is duplicated
```

Functions are values that can be stored in variables, and passed to and
returned from functions:
```
clac> sqrt
function

clac> magic = sqrt

clac> get_magic() = magic

clac> apply(f, x) = f(x)

clac> apply(get_magic(), 64)
8
```

Because functions are values, named functions are variables that contain a
function value:
```
clac> sqrt = -1
Error: variable 'sqrt' is already defined

clac> {sqrt(n) = n + 1, sqrt(36)}
37

clac> sqrt(36)
6
```

### Anonymous Functions
A function can be defined without a name by surrounding its parameters with
parentheses and defining its body with an arrow (`->`):
```
clac> () -> 3.14
function

clac> get_adder() = (l, r) -> l + r

clac> get_adder()(9, 10)
19
```

If an anonymous function only has one parameter, then the parentheses may be
omitted:
```
clac> apply_123(f) = f(123)

clac> apply_123((x) -> 2 * x)
246

clac> apply_123(x -> 3 * x)
369
```

### Function Scoping
Functions are lexically scoped, meaning they have access to the variables that
are in scope *where* they are defined, not *when* they are called:
```
clac> triple_value() = 3 * value, value = 123, triple_value()
Error: variable 'value' is undefined
```

Functions can access local variables defined outside of their body and
parameters. When they do this, they 'capture' the variables and become
closures:
```
clac> add(l, r) = l + r, subtract(l, r) = l - r

clac> curry(f, r) = l -> f(l, r)

clac> add_5 = curry(add, 5), subtract_10 = curry(subtract, 10)

clac> add_5(100), subtract_10(100)
105
90
```

### Built-in Functions
The Clac language includes built-in functions for commonly-used operations:
| Function                    | Usage                           |
| :-------------------------- | :------------------------------ |
| `sqrt(n: number) -> number` | Returns the square root of `n`. |

# Grammar
All valid Clac programs should have the following grammar:
```ebnf
program  = sequence, Eof ;
sequence = { stmt, [ "," ] } ;
stmt     = expr, [ "=", expr ] ;
expr     = expr_function ;

expr_function = expr_sum, [ "->", expr_function ] ;
expr_sum      = expr_term, { ( "+" | "-" ), expr_term } ;
expr_term     = expr_prefix, { ( "*" | "/" ), expr_prefix } ;
expr_prefix   = "-", expr_prefix | expr_call ;
expr_call     = expr_primary, { expr_paren } ;
expr_primary  = expr_paren | "{", sequence, "}" | Literal | Ident ;
expr_paren    = "(", [ expr, { ",", expr }, [ "," ] ], ")" ;
```

Programs that follow this grammar will be parsed successfully, but may fail
during semantic analysis or at runtime.

# Credits
* Infix parser based on
[pseudocode by Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing#precedence-climbing-how-it-actually-works).

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
