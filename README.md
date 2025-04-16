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
which are evaluated and printed in sequence.

The [EBNF](https://en.wikipedia.org/wiki/Extended_Backus-Naur_form) grammar
below is a reference for the language's syntax:
```EBNF
(* Clac Language Reference Grammar *)

(* A program is the CLI arguments space-delimited, or a REPL line. *)
program = { expr }, Eof ;

(* Every item in a program is parsed as an expression. The `expr` rule is used
as shorthand for the lowest precedence expression. *)
expr = atom ;

(* TODO: Infix expressions. *)

(* An atom is a high-precedence expression that can be used inside any infix
expression without needing parentheses. The implementation merges these rules
into one function for better performance. *)
atom         = atom_negate ;
atom_negate  = "-", atom_negate | atom_primary ;
atom_primary = "(", expr, ")" | Literal ;
```

# Goals
* [x] Read expressions in a loop.
* [x] Read command-line arguments as expressions.
* [x] Parse tokens from expressions.
* [ ] Parse abstract syntax trees (ASTs) from tokens.
* [ ] Evaluate ASTs.
* [ ] Allow variables to be defined and used.
* [ ] Allow functions to be defined and used.
* [ ] Add a library of intrinsic variables and functions (pi, sine, etc.)

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
