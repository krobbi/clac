[Go back](README.md)

# Grammar
All valid Clac programs should have the following grammar:
```ebnf
program  = sequence, Eof ;
sequence = { stmt, [ "," ] } ;
stmt     = expr ;
expr     = expr_assignment ;

expr_assignment = expr_mapping, [ "=", expr_mapping ] ;
expr_mapping    = expr_or, [ ( "->" | "?", expr, ":" ), expr_mapping ] ;
expr_or         = expr_and, { "||", expr_and } ;
expr_and        = expr_comparison, { "&&", expr_comparison } ;
expr_comparison = expr_sum, [ ( "==" | "!=" | "<" | "<=" | ">" | ">=" ), expr_sum ] ;
expr_sum        = expr_term, { ( "+" | "-" ), expr_term } ;
expr_term       = expr_prefix, { ( "*" | "/" ), expr_prefix } ;
expr_prefix     = ( "-" | "!" ), expr_prefix | expr_power ;
expr_power      = expr_call, [ "^", expr_prefix ] ;
expr_call       = expr_primary, { expr_paren } ;
expr_primary    = expr_paren | "{", sequence, "}" | Literal | Ident ;
expr_paren      = "(", [ expr, { ",", expr }, [ "," ] ], ")" ;
```

> [!NOTE]
> Not all programs which follow this grammar are valid. They may have semantic
> errors which are caught at compile time or at runtime.

> [!NOTE]
> Assignments are parsed as expressions to simplify compilation and improve
> error messages. Assignments are actually statements because they never
> produce a value.

> [!NOTE]
> Tuples are parsed to support parameter lists for anonymous functions. They
> are not supported as standalone values.
