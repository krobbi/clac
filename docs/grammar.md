[Go back](README.md)

# Grammar
All valid Clac programs should have the following grammar:
```ebnf
program  = sequence, Eof ;
sequence = { stmt, [ "," ] } ;
stmt     = expr, [ "=", expr ] ;
expr     = expr_mapping ;

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

Programs that follow this grammar will be parsed successfully, but may fail
during semantic analysis or at runtime.
