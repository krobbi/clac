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
expr = infix ;

(* An infix expression is an expression with an operand on each side of a
single operator. Infix expressions are grouped based on the mathematical order
of operations. The implementation uses precedence climbing for these rules for
better maintainability. *)
infix         = infix_sum ;
infix_sum     = infix_product, { ( "+" | "-" ), infix_product } ;
infix_product = atom, { ( "*" | "/" ), atom } ;

(* An atom is a high-precedence expression that can be used inside any infix
expression without needing parentheses. The implementation merges these rules
into one function for smaller code size and better performance. *)
atom         = atom_negate ;
atom_negate  = "-", atom_negate | atom_primary ;
atom_primary = "(", expr, ")" | Literal ;
```

# Goals
* [x] Read code in a loop.
* [x] Read command-line arguments as code.
* [x] Parse tokens from code.
* [x] Parse expressions from tokens.
* [x] Evaluate expressions.
* [ ] Allow variables to be defined and used.
* [ ] Allow functions to be defined and used.
* [ ] Add a library of intrinsic variables and functions (pi, sine, etc.)

# Credits
* Infix parser based on
[pseudocode by Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing).

# License
Clac is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
