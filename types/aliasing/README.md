# Aliasing

The type statement can be used to give a new name to an existing type. Types must have UpperCamelCase names, or the compiler will raise a warning. The exception to this rule are the primitive types: usize, f32, etc.

The main use of aliases is to reduce boilerplate; for example the IoResult<T> type is an alias for the Result<T, IoError> type.
