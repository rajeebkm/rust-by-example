# Enums

The enum keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as a struct is also valid as an enum.

## Type aliases

If you use a type alias, you can refer to each enum variant via its alias. This might be useful if the enum's name is too long or too generic, and you want to rename it.

The most common place you'll see this is in impl blocks using the Self alias.

To learn more about enums and type aliases, you can read the stabilization report (https://github.com/rust-lang/rust/pull/61682/#issuecomment-502472847) from when this feature was stabilized into Rust.
