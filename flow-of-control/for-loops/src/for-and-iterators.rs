//---for and iterators---//

// The for in construct is able to interact with an Iterator in several ways. As discussed in the section on the Iterator trait, by default the for loop will apply the into_iter function to the collection. However, this is not the only means of converting collections into iterators.

//iter, into_iter, and iter_mut all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.



fn main() {

//     // 1. iter - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);

    println!("//----------------------------------------------------------//" );


    // 2. into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),  //&"Ferris" will give ERROR ! expected `str`, found `&str`
            _ => println!("Hello {}", name),
        }
    }
    
    // println!("names: {:?}", names);  //ERROR ! value borrowed here after move


    println!("//----------------------------------------------------------//" );


    // 3. iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.

  
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

}




