* Leak a piece of data by never calling its destructor

Useful for things that are going to be used for the life of the program, but aren't technically
static (because they are created in response to arguments, environment, or other
configuration/data read at program start).

This is a modified version of the proposed rfc: https://github.com/rust-lang/rfcs/pull/1233

Notable changes:
- for user convenience, leak() is a non-static method
- Return `&T` instead of `&mut T`

While it would be ideal to return a `&'a mut T`, we apparently can't do that due to limitations
in rust's borrow checker causing soundness issues. Details are in the RFC liked above.
