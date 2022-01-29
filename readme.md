

# SicLib

[![License: MIT][mit-image]][mit-url] [![Github Actions][github-actions-image]][github-actions-url]
 <!-- [![CPP][cpp-image]][cpp-url] [![RUST][rust-image]][rust-url] -->

A Rust/C++/Haskell library with custom implementations to use in other projects.

## Design Philosophy

To freely experiment with both languages in a tightly coupled environment. Rust and
the C++ portion expose different interfaces, but are allowed to freely interop between
one another internally.

- siclibcpp is the namespace for the C++ inferface
- siclibrs is the namespace for the Rust interface
- siclibhs is the namespace for the Haskell interface
- siclibc is the namespace for the C interface (not first class)


## Installation

using CMake's FetchContent for siclibcpp OR Cargo for siclibrs.

## Example Concrete Implementations



[mit-image]: https://img.shields.io/badge/License-MIT-yellow.svg
[mit-url]: https://opensource.org/licenses/MIT

[github-actions-image]: https://github.com/ShameekConyers/siclib/actions/workflows/Build%20and%20Tests.yml/badge.svg?event=push
[github-actions-url]: https://github.com/ShameekConyers/siclib/actions

[cpp-image]: https://img.shields.io/badge/C%2B%2B-00599C?style=for-the-badge&logo=c%2B%2B&logoColor=white
[cpp-url]: ""

[rust-image]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: ""
