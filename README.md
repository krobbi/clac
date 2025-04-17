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
small scripting language. A Clac program consists of zero or more expressions,
which are evaluated and printed in sequence:
```
clac> 1 + 2 * 3  (1 + 2) * 3
7
9
```

## Variables
Variables can be defined or assigned with the `=` operator:
```
clac> x = 5  x * x  x = 2 * x
5
25
10
```

Variable names take the typical form of one or more ASCII letters or
underscores, with digits being allowed after the first character. All variables
are currently global.

Variable assignments currently return their assigned value, resulting in it
being printed. This may change in a future version.

## Grammar
The [EBNF](https://en.wikipedia.org/wiki/Extended_Backus-Naur_form) grammar
below is a reference for the language's syntax:
```EBNF
(* Clac Language Reference Grammar *)

(* A program is the CLI arguments space-delimited, or a REPL line. *)
program = { expr }, Eof ;

(* Every item in a program is parsed as an expression. The `expr` rule is used
as shorthand for the lowest precedence expression. *)
expr = infix ;

(* An infix expression is an expression with an operand on each side of a
single operator. Infix expressions are grouped based on the mathematical order
of operations and conventions from other languages. The implementation uses
precedence climbing for these rules for better maintainability. *)
infix            = infix_assignment ;
infix_assignment = infix_sum, [ "=", infix_assignment ] ;
infix_sum        = infix_term, { ( "+" | "-" ), infix_term } ;
infix_term       = atom, { ( "*" | "/" ), atom } ;

(* An atom is a high-precedence expression that can be used inside any infix
expression without needing parentheses. The implementation merges these rules
into one function for smaller code size and better performance. *)
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
