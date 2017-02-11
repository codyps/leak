# Leak a piece of data by never calling its destructor

Useful for things that are going to be used for the life of the program, but aren't technically
static (because they are created in response to arguments, environment, or other
configuration/data read at program start).

This is a modified version of the proposed rfc: https://github.com/rust-lang/rfcs/pull/1233

Notable changes:

- for convenience, leak() is a non-static method
