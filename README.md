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
clac [EXPRESSION]
```

If Clac is given one or more arguments, they are treated as a single expression
with spaces between the arguments. The expression is evaluated, and Clac exits
automatically.

If Clac is not given any arguments, the user can enter expressions in a loop
until manually exiting with `Ctrl+Z` (Windows) or `Ctrl+D` (other OS).

# Goals
* [x] Read expressions in a loop.
* [x] Read command-line arguments as expressions.
* [ ] Parse tokens from expressions.
* [ ] Parse abstract syntax trees (ASTs) from tokens.
* [ ] Evaluate ASTs.
* [ ] Allow variables to be defined and used.
* [ ] Allow functions to be defined and used.
* [ ] Add a library of intrinsic variables and functions (pi, sine, etc.)

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
