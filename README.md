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
so. There are currently 2 types:

### Number
The number type is the fundamental type of the Clac language. The type holds a
64-bit floating-point number, which is mostly used for evaluating mathematical
expressions.

### Void
The void type holds no value. Void can be returned by expressions to represent
returning no value.

To reinforce the concept of having no value, the void type has restrictions
that do not apply to other types:
* Void cannot be passed as an argument to an operator or a function.
* Void cannot be stored in a variable.
* Void cannot be printed.
* Void cannot be constructed with a literal value.

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
Runtime error: cannot use void as an argument

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
clac> 1(2, 3, 4)
TODO: Implement calls.
Called '1'
* With argument '2'
* With argument '3'
* With argument '4'
```

Functions do not yet exist in the Clac language, so a number value has to be
called as a placeholder.

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
* [ ] Add a library of intrinsic variables and functions (pi, sine, etc.)

# Credits
* Infix parser based on
[pseudocode by Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing).

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
