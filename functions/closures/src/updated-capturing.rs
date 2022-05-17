//---Capturing---//

// Closures are inherently flexible and will do what the functionality requires to make the closure work without annotation. This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing. Closures can capture variables:

// 1. by reference: &T
// 2. by mutable reference: &mut T
// 3. by value: T

// They preferentially capture variables by reference and only go lower when required.

fn main() {
    use std::mem;
    
    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time. 
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow = &color;
    print();

    println!("`reborrowed color`: {}",_reborrow);

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color; // move out of `color` occurs here

    // print();   //ERROR!  move out of `color` occurs here

    println!("`moved color`: {}", _color_moved);

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // // Call the closure using a mutable borrow.
    inc();
    // println!("`count`: {}", count);  // ERROR! immutable borrow occurs here

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;  // ERROR! immutable borrow occurs here
    // println!("{}", _reborrow );
    
    inc();      // mutable borrow later used here  

 

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &count; 
    println!("`count reborrowed:` {}", _count_reborrowed );

    
    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
  


// Using move before vertical pipes forces closure to take ownership of captured variables:

    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move|needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());  // value borrowed here after move
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}
