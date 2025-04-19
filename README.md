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
* Void cannot be passed as an argument to an operator.
* Void cannot be stored in a variable.
* Void cannot be printed.
* Void cannot be constructed with a literal value.

## Variables
Variables can be assigned with the `=` operator:
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
```

Variable names consist of one or more ASCII letters or underscores with digits
allowed after the first character. All variables are global.

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
atom_prefix  = "-", atom_prefix | atom_primary ;
atom_primary = "(", expr, ")" | Literal | Ident ;
```

# Goals
* [x] Read code in a loop.
* [x] Read command-line arguments as code.
* [x] Parse tokens from code.
* [x] Parse expressions from tokens.
* [x] Evaluate expressions.
* [x] Allow variables to be defined and used.
* [ ] Allow functions to be defined and used.
* [ ] Add a library of intrinsic variables and functions (pi, sine, etc.)

# Credits
* Infix parser based on
[pseudocode by Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing).

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
