[Go back](README.md)

# Functions
Functions are called by following a function with zero or more arguments
surrounded by parentheses:
```
clac> sqrt(25)
5
```

Functions must be called with the expected number of arguments:
```
clac> sqrt()
Error: incorrect number of arguments for function call

clac> sqrt(1, 2)
Error: incorrect number of arguments for function call
```

## User-defined Functions
Functions are defined with algebraic syntax. An expression defining the
function's body can be assigned to a 'call' that defines the function's name
and parameters:
```
clac> f(x) = 2 * x + 1

clac> f(1), f(2), f(3)
3
5
7
```

Function bodies must be an expression that produces a value:
```
clac> nop() = {}
Error: functions must return a value
```

Unlike [blocks](blocks.md) and the
[top level of the program](program_structure.md), function parameters and call
arguments *must* be separated with commas. Trailing commas may optionally be
used:
```
clac> sqdist(x0 x1) = {dx = x1 - x0, dx * dx}
Error: expected a closing ')', got identifier 'x1'

clac> sqdist(x0, x1,) = {dx = x1 - x0, dx * dx}

clac> sqdist(10 20)
Error: expected a closing ')', got number '20'

clac> sqdist(10, 20,)
100
```

The function's name and parameters must be defined with identifiers that are
not surrounded by parentheses. Each parameter must also have a different name
to every other parameter:
```
clac> 1(x) = x + 1
Error: function names must be identifiers

clac> (f)(x) = x + x
Error: function names must be identifiers

clac> f(1) = 2
Error: function parameters must be identifiers

clac> f((x)) = 1 / x
Error: function parameters must be identifiers

clac> f(x, x) = x * x
Error: function parameter 'x' is duplicated
```

## Functions are Values
Functions are values that can be stored in [variables](variables.md), and
passed to and returned from functions:
```
clac> sqrt
function

clac> magic = sqrt

clac> get_magic() = magic

clac> apply(f, x) = f(x)

clac> apply(get_magic(), 64)
8
```

Because functions are values, named functions are variables that contain a
function value:
```
clac> sqrt = -1
Error: variable 'sqrt' is already defined

clac> {sqrt(n) = n + 1, sqrt(36)}
37

clac> sqrt(36)
6
```

## Anonymous Functions
A function without a name can be defined with the `->` operator, with the
function's parameters on the left-hand side of the operator and the function
body on the right:
```
clac> () -> 3.14
function

clac> get_adder() = (l, r) -> l + r

clac> get_adder()(9, 10)
19
```

If an anonymous function only has one parameter, then the parentheses may be
omitted:
```
clac> apply_123(f) = f(123)

clac> apply_123((x) -> 2 * x)
246

clac> apply_123(x -> 3 * x)
369
```

## Function Scoping
Functions are lexically scoped, meaning they have access to the variables that
are in scope *where* they are defined, not *when* they are called:
```
clac> triple_value() = 3 * value, value = 123, triple_value()
Error: variable 'value' is undefined
```

## Closures
Functions can access local variables defined outside of their body and
parameters. When they do this, they 'capture' the variables and become
closures:
```
clac> add(l, r) = l + r, subtract(l, r) = l - r

clac> curry(f, r) = l -> f(l, r)

clac> add_5 = curry(add, 5), subtract_10 = curry(subtract, 10)

clac> add_5(100), subtract_10(100)
105
90
```
