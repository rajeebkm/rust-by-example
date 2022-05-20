# Modules

Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and manage visibility (public/private) between them.

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

## Visibility

By default, the items in a module have private visibility, but this can be overridden with the pub modifier. Only the public items of a module can be accessed from outside the module scope.

## Struct visibility

Structs have an extra level of visibility with their fields. The visibility defaults to private, and can be overridden with the pub modifier. This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation).

## The use declaration

The use declaration can be used to bind a full path to a new name, for easier access. It is often used like this:

use crate::deeply::nested::{
my_first_function,
my_second_function,
AndATraitType
};

fn main() {
my_first_function();
}

You can use the as keyword to bind imports to a different name:

// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() {
println!("called `function()`");
}

mod deeply {
pub mod nested {
pub fn function() {
println!("called `deeply::nested::function()`");
}
}
}

fn main() {
// Easier access to `deeply::nested::function`
other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();

}

## super and self

The super and self keywords can be used in the path to remove ambiguity when accessing items and to prevent unnecessary hardcoding of paths.

## File hierarchy

Modules can be mapped to a file/directory hierarchy.
