/*This code consists of functions that contain basic variable definitions in Rust. 
Uncommnet the function you want to examine from the main function and run the code. */

fn mutable_variables(){
    /*In Rust, variables are mutable by default. That is,   
    once a value is assigned during definition, the value cannot be changed again.*/

    let x = 5; // 
    println!("the value of x : {}", x); // it prints the value 5

    //uncomment the next line and see geting an compiler error becouse x is mutable. Can't be change
    //x = 6; 

    //Shadowing in Rust
    let x = 6; //We define a new variable with a previously defined variable name.
    println!("the value of x : {}", x); // Getting no error, it prints the value 6.

    // We can also change the type of variable by Shadowing
    let x = "Hello World"; 
    println!("the value of x : {}", x); // it prints "Hello Word"
}

//If you want to create a non-mutable variable use mut keyword
fn non_mutable_variables(){ 
 
    let mut y = 10;
    println!("the value of y : {}", y); // it prints the value 10

    y = 15; // Getting no error, because y is non_mutable. 
    println!("the value of y : {}", y); // it prints the value 10
}

fn cont_variables(){
    //Constants are identified by capital letters as a general practice
    //When defining a constant, the type must also be specified. Type is not automatically detected.
    const MY_FIRST_CONST : u16 = 20;

    //Underscores can be used to make large numbers readable
    const MY_SECOND_CONST : u32 = 5_000_000;
    
    //The mut keyword cannot be used. Uncomment the next line and see the compile error
    //const mut MY_THIRD_CONST :i32 = 35; 

    //A constant variable cannot be assigned the return value of a function or expression
    let x = 10;
    //uncomment the next line and see the error
    //const MY_FOURTH_CONST : i16 = 5 + x;
    

    println!("the value of constant is  : {}", MY_FIRST_CONST); // it prints the value 20
    println!("the value of constant is  : {}", MY_SECOND_CONST); // it prints the value 20
}


fn main(){

    //mutable_variables();
    //non_mutable_variables();
    //cont_variables();

}