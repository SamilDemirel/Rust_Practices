/* Rust Module System
    - Package
        - Crates (binary or library) main.rs is the root crate of the package
            - Modules    
        
**if we create a new package on terminal with :
    /cargo new --lib my_package 
    package will create with lib.rs in src folder. not with main.rs and a test module automaticly added
 */

 /*this file is not including a working code.
its more like lecture notes about module system in rust. 

mod front_of_house{
    mod hosting{

        fn add_to_wait_list(){}

        fn seat_at_table(){}

    }

    mod serving{

        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}

    }

}

pub fn eat_at_retaurant(){
    //if we need to use the add_to_wait_list function we need to defi the path of the function by

    //Absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    //or Relative Path. it starts from current modul
    /*current modul::*/front_of_house::hosting::add_to_wait_list();

}
//we are getting errors at hosting because, hosting module is private not public

PRIVACY RULE!!!!!
Functions, structs, constants, etc., defined inside a module (mod) 
are not accessible from other modules (especially the parent module) by default. 

//if we want to access to add_to_wait_list function from root module,

mod front_of_house{
    pub mod hosting{   //we added pub key word and made it public

        pub fn add_to_wait_list(){}  //we added pub key word and made it public

        fn seat_at_table(){}

    }

    mod serving{

        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}

    }

}

fn serve_order(){} //defining a function on crate

mod back_of_house{ // defining a module (this one is a child modul for crate) 
    
    fn fix(){
        cook();
        super::serve_order(); // calling the parent modul's function by super:: 
    }

    fn cook(){}
}

// With the Structs, struct it self and all the variables and methods must be public to reach out of the module
// With the Enums, The enum itself being public is sufficient for external access to its variables and methods.


//---------------------------------- use Key Word-------------------

crate::front_of_house::hosting::add_to_wait_list();

//if we use use keyword tobring the module to scop

use crate::front_of_house::hosting;

//now we can call the function directly

hosting::add_to_wait_list();

//If you need to use use for two different modules that have modules with the same name, then one of the modules must be replaced with 'as' keyword.
use std::fmt::Result;
use std::io::Result as IoResult;

//if we need to use more than one modules of one crate or module, we can use {}
use rand::{Rng, CyrptoRng, ErrorKind::Transient};

//if we need to use a module and a sub modul of that modul we can use self keyword
use std::io::{self,Write}; // it means use io and also io::write

//if we want to re-export a module (it means open a module outside useage)
pub use std::io::Write; 

//if we need to use all modules we can use '*'
use std::io::*;

mod front_of_car{
    pub mod host{
        pub fn add_to_list(){}
    }
}

we decided tocary all front_of_car module to a new file
    1. create a new file as front_of_car.rs !! must be same name
    2. cut the code inside the mod front_of_car{}
    3. mod front_of_car{
            pub mod host{
                pub fn add_to_list(){}
            }
        } // change this code to
       
       mod front_of_car; 
//now we have a difrent file with name front_of_car.rs with content
        
        pub mod host{
            pub fn add_to_list(){}
        }

    if we need to move the content of the mod host to another file, 
    we must create a new folder with same name of front_of_car.rs
    in this folder we must create a new file with name of host.rs move all content in pub mod host{} here
    in the front_of_car.rs change the content to

        pub mod host;
    
    //the structure will look like this
    -src
        -front_of_car
            -host.rs
        -fron_of_car.rs
        -lib.rs

*/  


fn main(){



}