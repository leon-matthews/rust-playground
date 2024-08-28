
# Native Python Modules

Rust is an ideal language for accelerating small parts of a Python project. Its
performance is fantastic, its memory, type safety, and multithreading
capabilities are all top-notch - but it is the well thought-out tooling that
really makes using it a pleasure.


## The `PyO3` and `Maturin` Libraries.

The [`PyO3`](https://pyo3.rs/) Rust library supports either writing a native Python module in Rust,
or to embed Python in a Rust binary. My interest is in the former - to
accelelate a large Python project by accelelating its hot points using native
modules written in Rust.


## Getting Started

The [`Maturin`](https://maturin.rs/) library makes this use case relatively
easy. It is a tool for building and publishing Rust-based Python packages.

    $ workon adder
    $ pip install maturin
    $ cd ~/Projects/rust/
    $ maturin new -b pyo3 adder

The `maturin new` command is just like `cargo new`, but adds the required
boilerplate needed to both *Cargo.toml* and a new *pyproject.toml* that will
instruct the Python ecosystem how to install our native module.

The crate type has been changed to a `cdylib` library type. Running `cargo
build` will now build an `so` shared library file that can be linked into C/C++
programs.


## Rust Code

In the top-level library `lib.rs` we include the *PyO3* prelude, then use its
macros to mark the code we which to expose as part of a Python module. We have
three choices:

1. Python modules, via the `#[pymodule]` macro
2. Python functions, via the `#[pyfunction]` macro
3. Python classes, via the `#[pyclass]` and `#[pymethods]` macros.

We need at least one module and one function. The boilerplate provided by
`maturin new` has done that for us, providing a function that adds two integers
then converts the result to a string, and a Python module with the same name
as our rust library.


## Install Rust Module into Python Virtualenv

Simply running `maturin develop` inside the Rust project will cause the Python
module to be built and installed into the current Python virtual enviroment:

    $ maturin develop
    ...
    🛠 Installed adder-0.1.0

    $ pip list
    Package       Version Editable project location
    adder         0.1.0   ~/Projects/rust-playground/python/adder
    ...

    $ python3
    >>> import adder
    >>> adder.sum_as_string(97, 997)
    '1094'
    >>> help(adder)
    ...


# Build Python Wheel

It is as easy as running:

    $ maturin build --release


## Rust Types and Python Data

If you've ever wrestled with the Rust compiler, you'll know how much it cares
about type safety. Luckily, the *PyO3* library provides ways to easily pass
data between your Rust library and the Python runtime.

For example, take a look at the example function provided by `maturin new`:

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

There's a lot to see here. Firstly, the Rust docstring is copied in as the
Python docstring for the function, which Python users can read using `help()`,
etc.

Next, the function's arguments are `usize`, Rust's native unsigned integer type,
32- or 64-bit, depending on the underlying platform. *PyO3* converts a Python
integer to this automatically. What's really nice is that errors are taken
care of too. Passing a too-big integer into our sample function from the Python
side doesn't cause our Rust code to crash, rather an `OverflowError` is raised.

    >>> adder.sum_as_string(2**63, 1)
    '9223372036854775809'
    >>> adder.sum_as_string(2**64, 1)
    ...
    OverflowError: int too big to convert

Passing in invalid types results in a `TypeError` being raised, and with a
sensible error message too!

    >>> adder.sum_as_string('banana', 'carrot')
    ...
    TypeError: argument 'a': 'str' object cannot be interpreted as an integer

Note that Rust can support Python's unlimited precision integers using the
[`num_bigint`](https://crates.io/crates/num-bigint) crate.

Finally, the return value is `PyResult<String>`. Here we are returning a native
Rust `String` in a *PyO3* `PyResult` type. The `String` will be converted to a
Python `str`, and the `PyResult` allows us to raise a Python exception if the
`Err` variant is returned on the Rust side.


### To Convert or Not?

You don't actually *have* to convert from Python to Rust types in your functions,
as *PyO3* can work with native Python types, but I strongly suggest that you do.

Doing your conversions at the boundaries of your code always results in cleaner
code and easier to understand APIs (think Unicode or timezone conversions), and
in the common case where you're doing significant processing on the Rust side,
the cost of conversion in minimal.

