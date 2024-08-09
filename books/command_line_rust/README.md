
# Command-Line Rust (1st Ed., 2022)

## A Project-Based Primer for Writing Rust CLIs

*Ken Youens-Clark*


## Resources

[Code Examples](https://github.com/kyclark/command-line-rust)


## Currently

Finished using the *build* API from `clap` to parse the command-line arguments.


## Next

Write tests for argument parsing, refactoring current approach to allow for
testing as needed.


## Key Techniques

### wcr

  - Use `std::io::cursor` to create file-like objects for testing in the
    function `test_count()` in *lib.rs*.
