//There are two build modes in Cargo
    //1. Dev  $cargo build
        // compiles quickly and requires little optimization (opt-level =0)
        // Suitable for debugging.
        //Output: placed in target/debug/ directory.

    //2. Release $cargo build --release
        // It compiles slower because it is highly optimized (opt-level = 3).
        //In terms of performance, a much faster binary is obtained.
        //Debug information is mostly disabled.
        //Output: placed in the target/release/ directory.
        //Used before publishing or distribution.

// we can change opt-level manualy on cargo.toml by adding:
    // [profile.dev]
    // opt-level = 0 or 1 or 2 or 3 (default = 0)
    // [profile.release]
    // opt-level = 0 or 1 or 2 or 3 (default = 3)


/* For Publishing a Crate on crate.io
   1. Documentation should be prepared by using '///' for a spesific item (functions, structs, enums etc..)
      and '//!' for all Modul or create (It is usually written at the beginning of a file or crate)
   2. Cargo prepare an html file by using these documentation
   3. the code between  two ``` runs as test by cargo
   4. To open the html format of documentation
        $cargo doc --open
*/


/* Re-Exporting 

if we have a modul that:
    mod internal {
    pub mod math {
        pub fn sum(a: i32, b: i32) -> i32 {
            a + b
        }
    }
}

the user must use my_crate::internal::math::sum() to use my function

if we re-export the sum function by adding this line
    pub use my_crate::internal::math::sum; 
user can call the sum function by this line
    use mycrate::sum;

*/

/// Adds two numbers and retruns result
/// 
/// #Examples //this is a section to test 
/// 
/// ```
///  let a = 10;
///  let b= 5;
///  let total = best_sum(&a, &b);
///  assert_eq!(17, total);
/// ```
///  
/// #Panics //this section explains the scenarios in which function being documented could panic
/// #Errors //this section explains if the function retruns a Result, describing the kinds of errors might occur
/// #Safety //this section explains if the function is unsafe to call 

fn best_sum(num1:&i32, num2:&i32) -> i32{
    num1 + num2   
}

    fn  main(){
        let a = 10;
        let b= 5;

        let total = best_sum(&a, &b);
        println!("Total is: {}", total);

}