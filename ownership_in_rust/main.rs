/*
-------Ownership Rules of Rust------------------------

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

*/



fn stack_and_heap(){
    //fixed sized variables(integer, float, bool, char, string literal) stored in stack. 
    let x =5;
    //dynamic sized variables's pointers stored in stack
    //but the values of the variables stored in heap
    let y = String::from("Hello");// creating new instance of String and putting it to the heap
    println!("The value of the x : {}", x);
    println!("The value of the y : {}", y);

}

fn copying_variables(){
    let x =5;
    let y = x; //direcly creating a copy of value at stack
    println!("The value of the x : {}", x);
    println!("The value of the y : {}", y);

    //in the next two lines both s1 and s2 pointers in stack shows the same String value on the heap
    //not creating a new string on the heap.
    let s1 = String::from("Hello");
    let s2 = s1; // it should be a shallow copy but rust does not aloow that 

    //next line give an error because after s1 dropped by Rust due to RULE NUMBER 2 
    //println!("The value of the s1: {}", s1);
    //s2 borrowed s1's value which is on the heap ans s1 dropped
    println!("The value of the s2 : {}", s2);

    // if we want to use same string with two different variables
    let s3 = String::from("World");
    let s4 = s3.clone();

    println!("The value of the s3 : {}", s3);
    println!("The value of the s4 : {}", s4);
}


fn ownership_on_heap(){

    fn take_ownership(my_string: String){
        println!("{}", my_string);
    }

    fn take_and_give_ownership(my_string: String) -> String{
        println!("{}", my_string);
        my_string
    }

    let s =String::from("hello");
    //take_ownership(s);

    /*when this function called at the background rust makes let my_string = s; 
    So s drops, only my_string points to "hello" string on heap
    and next line gives error. */
    
    //println!("{}",s)

    let s = take_and_give_ownership(s);
    /*when we call this function, in the function let my_string = s and s drops
    but at the end of the function it return my_String value back to shadowed s
    so s get the ownership back of the String and we can use it without an error*/
    println!("{}",s)
}

fn ownership_on_stack(){

    fn take_ownership(my_int: i32){
        println!("{}", my_int);
    }

    let i = 15;
    // i is an integer so it is on the stack. 
    //when called the function at the background rust makes let my_int = i;
    //fixedsized values can be coppied so the i does not drop and steel can be use
    take_ownership(i);
    println!("{}",i)
}

fn pass_by_reference(){

    /*Instead of passing the variable s directly to the function and giving ownership of s to my_string,
     we pass the reference of s to the function. by adding '&' char infront of the variable name*/

    fn print_string(my_string : &String){ //my_string is a String referance
        println!("The Value of my_string : {}", my_string); // it prints the value of string
        println!("The Heap Adress of my_string : {:p}", my_string) // it prints the Heap Adress of string

    }

    let s = String::from("Hello");
    print_string(&s);
    println!("{}", s); //s still have the ownership without giving back
}

//The references in Rust are default Inmutable
//if we want to change a variable's value on the heap

fn change_by_reference(){

    fn change_string(old_string: &mut String){ //String reference parameter must be &mut!!
        old_string.push_str(", World");
    }

    let mut my_string = String::from("Hello"); //String variable must be mut!!
    println!("Old String Value : {}", my_string);

    change_string(&mut my_string); //Argument must be &mut
    println!("New String Value : {}", my_string);
}


/*
-------References Rules of Rust------------------------
1. At any given time, you can have either one mutable reference
or any number of imutable references.
2. References always be valid
*/


/*ATTENTION: one mutable variable can have only one mutable reference due to rule 1
let mut s = String::from("Hello");

let reference1 = &mut s;
let reference2 = &mut s; // this line gives error to keep the memory safe */

/* we can have more than one inmutable reference for a mutable variable
let mut s = String::from("Hello");

let reference1 = &s;
let reference2 = &s; / no error. Because the heap data cant be change by these inmutable references. Memory is safe

//if we add a new mutable reference with next line
let r3 = &mut s; //this line gives error. no permision for that to keep memory safe */

fn safely_multiple_references(){
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    //let r3 = &mut s; //This line gives error 

    println!("s value by r1 {}, s value by r2 {}", r1, r2);

    //after this line r1 and r2 are done and dropped. because this is the last line they were used in.
    //so now we can define a new mutable reference without error

    let r3 = &mut s;
    r3.push_str(" World");

    println!("New value of s : {}", s);
}

/* uncomment the function to see the error
fn dangling_refs(){

    fn return_dangle_ref() -> &String{
        let my_string = String::from("Hello");
        
       //next line try to return the refence of my_String's but my_string's scop is over so &my_string points invalid data
       //so the line gives error due to rule 2
       &my_string 

       //Rust does not allow references to invalid data points. 
       //So when you uncomment the line, you see an error 
    }

    let my_reference = return_dangle_ref();
}*/




fn main(){
   
    //stack_and_heap();
    //copying_variables();
    //ownership_on_stack();
    //ownership_on_heap();
    //pass_by_reference();
    //change_by_reference();
    safely_multiple_references();

}