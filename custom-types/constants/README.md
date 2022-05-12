# constants

Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:

1. const: An unchangeable value (the common case).
2. static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.
