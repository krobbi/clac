# Clac
Clac was originally created as a command line calculator to address some
annoyances with the default Windows calculator:
* Relatively long time to start up
* No support for custom functions and variables
* No way to evaluate an expression from a command line session

The decision to support functions and variables expanded the project's scope to
being somewhere between a calculator and a small, mathematical scripting
language.

## Usage
Clac is run from the command line:
```shell
clac [CODE]
```

If one or more arguments are given, then they are joined with spaces and
treated as a single line of code. Clac executes the code and exits
automatically.

If no arguments are given, then the user can enter code in a loop until
manually exiting with `Ctrl+D` (Linux, macOS, etc.) or `Ctrl+Z` (Windows.)

Clac is designed to be usable as a calculator, so writing an expression at the
top level of a program will print its result:
```
clac> 1 + 1
2
```

> [!NOTE]
> For more information about language features, see the
> [language documentation](docs/README.md).

## License
Clac is released under the MIT License. See [LICENSE.txt](LICENSE.txt) for a
full copy of the license text.
