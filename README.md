# Clac
Command-line calculator.

# Contents
1. [About](#about)
2. [Usage](#usage)
3. [Goals](#goals)
4. [License](#license)

# About
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
until manually exiting.

Clac is not yet fully implemented and prints the given expression instead of
evaluating it.

# Goals
* [x] Read arithmetic expressions in a loop.
* [x] Read optional command-line arguments as expressions.
* [ ] Parse tokens from expressions.
* [ ] Parse expression trees from tokens.
* [ ] Evaluate expression trees.
* [ ] Add a context for defining and using variables.
* [ ] Add support for defining and using functions to the context.
* [ ] Add a library of built-in functions and variables (square root, pi, etc.)

# License
Clac is released under the MIT License:  
https://krobbi.gitub.io/license/2025/mit.txt

See [LICENSE.txt](/LICENSE.txt) for a full copy of the license text.
