# Literals and operators

Integers 1, floats 1.2, characters 'a', strings "abc", booleans true and the unit type () can be expressed using literals.

Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: 0x, 0o or 0b.

Underscores can be inserted in numeric literals to improve readability, e.g. 1_000 is the same as 1000, and 0.000_001 is the same as 0.000001.

We need to tell the compiler the type of the literals we use. For now, we'll use the u32 suffix to indicate that the literal is an unsigned 32-bit integer, and the i32 suffix to indicate that it's a signed 32-bit integer.

The operators available and their precedence in Rust are similar to other C-like languages.
