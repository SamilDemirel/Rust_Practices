
use rust_practices; // this line take the lib.rs in the project to the scope.
//we dont need to define a module. Because cargo knows that all the files in test folder are a crate
//We can directly write our test

mod common; //it gets the mod.rs in common folder to the scope
            // it looks for a file with the name with mod.rs or common.rs


#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, rust_practices::add_two(2)); //we can call only the public functions
}

//When we run the test, it runs in three sections
    //1. Unit tests
    //2. Integration test
    //3. Document tests 

// if we want to run just integration test /& cargo test --test integration_test

/* If you have more than one integration test file in the test folder and code is shared between them, 
   problems may occur when the test is run because cargo will consider all the files in the test folder as crates.

   best practice for adding multiple files to test folder is adding the files in a new folder 
   because, the files in subdirectory do not compiles as crate
*/

//if we have a main.rs in src folder, it means we have a binary crate
//and we can not test binary crates directly
//It is common to see a binary crate wrapping a lib crate
