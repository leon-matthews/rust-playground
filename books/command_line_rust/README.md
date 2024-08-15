
# Command-Line Rust (1st Ed., 2022)

## A Project-Based Primer for Writing Rust CLIs

*Ken Youens-Clark*


## Resources

[Code Examples](https://github.com/kyclark/command-line-rust)


## Currently

Continue with `uniqr` program from Chapter 6.


## Key Techniques

### wcr

  - Use `std::io::Cursor` to create file-like objects for testing in the
    function `test_count()` in *lib.rs*.
  - Read lines from file using the same string buffer,
    ie. `file.read_line(&mut line)?`

### uniqr
