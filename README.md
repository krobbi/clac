# Clac
Clac is a command-line calculator that is being created as a personal project
to replace the default Windows calculator and address some minor annoyances
with it:
* Relatively long time to start up.
* No support for custom functions and variables.
* No way to evaluate an expression from a command-line session.

# Usage
Clac is run from the command-line:
```shell
clac [CODE]
```

If Clac is given one or more arguments, they are treated as a single piece of
code with spaces between the arguments. The program is run, and Clac exits
automatically.

If Clac is not given any arguments, the user can enter code in a loop until
manually exiting with `Ctrl+Z` (Windows) or `Ctrl+D` (other OS.)

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
Variables can be declared or assigned with the `=` operator:
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
Each block creates a new scope. Variables declared inside a block cannot be
used after the block:
```
clac> global = 5, global, {local = 2 * global, local}, local
5
10
Runtime error: variable 'local' is undefined
```

When an undeclared variable is assigned, the assignment first attempts to
assign an existing variable in a parent scope. Because there is not yet a
distinction between declaring and assigning a variable, it is impossible to
declare a variable that shadows a variable name:
```
clac> shadow = 0, {shadow = 1}, shadow
1
```

## Functions
Functions can be called by following a function expression with zero or more
arguments surrounded by parentheses and separated by commas:
```
clac> sqrt(64)
8
```

Functions are first-class values, meaning they can be assigned to variables,
and passed to and returned from functions. This also means that function calls
do not need to call a function name directly:
```
clac> s = sqrt, s
function

clac> s(sqrt)
Runtime error: incorrect argument types for operation

clac> s()
Runtime error: incorrect argument count for function

clac> s(1, 2)
Runtime error: incorrect argument count for function

clac> {op = sqrt, op}(2)
1.4142135623730951
```

It is not yet possible to define functions, but this is a planned feature.

### Built-in Functions
The Clac language includes built-in functions for commonly-used operations:
| Function                    | Usage                           |
| :-------------------------- | :------------------------------ |
| `sqrt(n: number) -> number` | Returns the square root of `n`. |

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

# Goals
* [x] Read code in a loop.
* [x] Read command-line arguments as code.
* [x] Parse tokens from code.
* [x] Parse expressions from tokens.
* [x] Evaluate expressions.
* [x] Allow variables to be defined and used.
* [x] Allow functions to be called.
* [ ] Allow functions to be defined.

# Credits
* Infix parser based on
[pseudocode by Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing).

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
