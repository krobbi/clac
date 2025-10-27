# Clac
Clac was originally created as a command line calculator to address some
annoyances with the default Windows calculator:
* Relatively long time to start up
* No support for custom functions and variables
* No way to evaluate an expression from a command line session

The decision to support functions and variables made the scope of the project
more complex. Clac now aims to be an interpreter for a small, mathematical
scripting language.

# Usage
Clac is run from the command line:
```shell
clac [CODE]
```

If Clac is given one or more arguments, then they are joined with spaces and
treated as a single line of code. Clac executes the code and exits
automatically.

If Clac is not given any arguments, then the user can enter code in a loop
until manually exiting with `Ctrl+D` (Linux, macOS, etc.) or `Ctrl+Z`
(Windows.)

Currently, Clac prints the input code as an abstract syntax tree.

<!--
# Language
Clac aims to implement a language that is somewhere between a calculator and a
small scripting language. A Clac program consists of zero or more expressions
separated by commas:
```
clac> 1.5 + 2 * 3, (1.5 + 2) * 3
7.5
10.5
```

The expressions are evaluated in order, and their returned values are printed.

## Types
The Clac language is dynamically typed. Expressions can return different types
of values, and the types are only checked at runtime when it is necessary to do
so. There are currently 3 types:

### Number
The number type is the fundamental type of the Clac language. The type holds a
64-bit floating-point number, which is mostly used for evaluating mathematical
expressions.

### Function
The function type holds a function that can be called with zero or more
argument values to return an optional result value. Being a type of value
allows functions to be passed to and returned from other functions.

### Void
Void is the type of no value. Some expressions may return no value to indicate
a side-effect or a lack of a meaningful value to return.

The void type has some restrictions that do not apply to other types:
* Void cannot be used as a value in an expression's input.
* Void cannot be stored in a variable.
* Void is not printed when it is returned from a top-level expression.

## Variables
Variables can be defined or assigned with the `=` operator:
```
clac> x = 5, x * x, x = 2 * x
25

clac> x
10
```

Variable assignments are not printed because they do not return a value. This
also means that variable assignments cannot be chained:
```
clac> x = y = 1
Runtime error: cannot use void as a value

clac> x
Runtime error: variable 'x' is undefined

clac> y
1
```

Variable names consist of one or more ASCII letters or underscores with digits
allowed after the first character.

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

# Grammar
All valid Clac programs should have the following grammar:
```ebnf
program = { expr, [ "," ] }, Eof ;
expr    = expr_sum ;

expr_sum     = expr_term, { ( "+" | "-" ), expr_term } ;
expr_term    = expr_prefix, { ( "*" | "/" ), expr_prefix } ;
expr_prefix  = "-", expr_prefix | expr_primary ;
expr_primary = "(", expr, ")" | Number ;
```

<!--
## Grammar
```EBNF
program = sequence, Eof ;
sequence = [ expr, { ",", expr } ] ;
expr = infix ;

infix            = infix_assignment ;
infix_assignment = infix_sum, [ "=", infix_assignment ] ;
infix_sum        = infix_term, { ( "+" | "-" ), infix_term } ;
infix_term       = atom, { ( "*" | "/" ), atom } ;

atom         = atom_prefix ;
atom_prefix  = "-", atom_prefix | atom_call ;
atom_call    = atom_primary, { "(", sequence, ")" } ;
atom_primary = "(", expr, ")" | "{", sequence, "}" | Literal | Ident ;
```
-->

# Credits
* Infix parser based on
[pseudocode by Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing#precedence-climbing-how-it-actually-works).

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
