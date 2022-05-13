//---Returning from loops---//

// One of the uses of a loop is to retry an operation until it succeeds. If the operation returns a value though, you might need to pass it to the rest of the code: put it after the break, and it will be returned by the loop expression.

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {}", result);

    
    assert_eq!(result, 20); //Asserts that two expressions are equal to each other (using PartialEq).
    // On panic, this macro will print the values of the expressions with their debug representations.
    //Like assert!, this macro has a second form, where a custom panic message can be provided.
    assert_eq!(result, 20, "We are checking result {} with the value {}", result, 20);
}
