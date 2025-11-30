[Go back](README.md)

# Blocks
Zero or more expressions and statements can be grouped into a block by
surrounding them with curly braces (`{}`). The items in a block can be
separated with commas using the
[same rules as the top level of the program](program_structure.md).

If a block ends with an expression, then the block is an expression that
produces the value of its last expression:
```
clac> {1, 2, 3}
3
```

Expression blocks allow statements to be used in a context where only
expressions are allowed.

If a block is empty or ends with a statement, then the block is a statement
that does not produce a value:
```
clac> 123, {} {4, x = 5} {6, {}} 789
123
789
```

## Block Scopes
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
