
fn dangling_reference(){

    let r =8;

    //defining a new scope 
    {
        let x =5;

        //uncomment the next line and see the error
        //r = &x;  

    }// end of the scope x is no more alive.
    //when x dropped, the reference of x (&x) can show any data in memory, its a dangling reference now.

    println!("r: {}", r);

    //Rust Borrow Checker runs at compile time.
    //It Checks if all borrowed values ​​or references are valid
}

fn normal_reference(){

    let x = 5;
    let r = &x;
    println!("r:{}", r); // x and r are still alive here, so this code does'nt give an error
}
fn requirement_to_life_time(){
    
    //if we dont specify life times, we get an error at retrun type
    //Because the Borrow Checker cant know if returned reference wont be a dangling reference
    fn longest(x: &str, y:&str) -> &str{ 
        //In this function, which x and y will be returned depends on the condition. 
        //1. problem : the lifetimes of x and y are different because they are in different scopes.
        //2. problem : Depending on where the function will be called from and with what values ​​it will be called, 
        //             the lifetimes of the values ​​that x and y refer to cannot be known.
        //Borrow Checker cannot handle these uncertainties and asks us to specify life times
        if x.len() > y.len(){
            x
        } else {
            y
        }
    }

    //Defining Generic Life Times
    //&'a i32 a reference with lifetime
    //&'a mut i32 a mutable reference with a lifetime

    fn longest_with_generic_life_time<'a>(x: &'a str, y:&'a str) -> &'a str{ 
        //this says to Borrow Checker that " x, y, and return value have the same life time!!"
        //defining life times does not chance life times, it only defines the relationship with life times 
        //we acually say here that "there is a relationship between z, y and return value" 
        //This Relationship is, "The lifetime of the returned reference is the same as the minimum lifetime of the variables." 
        //Depending on the condition and where the function is called;
            // if x has a minimum lifetime, the lifetime of the returned reference will be equal to that of x.
            // or if y has a minimum lifetime, the lifetime of the returned reference will be equal to that of y.
        if x.len() > y.len(){
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

    
fn different_life_times(){
   
    fn longest<'a>(x: &'a str, y:&'a str) -> &'a str{ 

        if x.len() > y.len(){
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    //let result: &str ;

    {
         let string2 = String::from("xyz");
         let result = longest(string1.as_str(), string2.as_str());
         println!("The longest string is {}", result);
         
    }
    // this time the shortest life time is string2's lifetime
    //so the longest functions returned reference's lifetime is equal to string2's life time
    
}

//The lifetime of the returned reference is always relative to the lifetime of the arguments.
// if we try to return a reference of a value which created inside the function
//    fn my_func<'a>(x: &str, y: &str) -> &'a i32 {
//        let x =23;
//         &x } we get an error. because once the function is over, the local variable will drop and the reference will be dangling

fn struct_with_life_time(){
//if we want to use references in a struct, we have to define lifetime
//it means, struct can not live more than the reference which passed in.
//in other words, struct's life time is equal the life time of the reference which passed in  
    struct Excerpt<'a>{
        part: &'a str,
    }

    let novel = String::from("call me osman. some years ago. hebele");
    let first_sentence  = novel.split('.').next().expect("No sentene found");
    let first_part = Excerpt{
        part:first_sentence,
    };
    //if we try to use first_part struct after the scope of the first_sentence we get an error
}

//-------Lifetime Illusion Rules--------------------
/*input lifetimes --> parameter's life times
  output lifetimes --> returnes reference's lifetime

1. Each parameter that is a reference gets it's own lifetime parameter
    //In this case lifetimes must be defined

2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

      fn get_first(x:&str) -> &str{} 
    //No error becuase Borrow Checker assumes that 
      fn get_first<'a>(x:&'a str) -> &'a str{}

3. If there are multiple input lifetime parameters, but one one them is &self or &mut self, 
   the lifetime of self is assigned to all output lifetime parameters.( this rule is for implemented methods)

    impl MyStruct {
    fn name(&self, other: &str) -> &str {
        self.field
    }

    //No error becuase Borrow Checker assumes that 
    fn name<'a>(&'a self, other: &str) -> &'a str
}
*/

fn static_lifetime(){
    //The lifetime of a reference declared as "static" is for the entire program.
    // all &str are static because they stored in the binary of the program

    let m = "i have a static life time without using 'static"; //default
    let s: &'static str= "i have a static lifetime"; //this is the syntax
    

}

fn main() {
    //normal_reference();
    requirement_to_life_time()
    
}