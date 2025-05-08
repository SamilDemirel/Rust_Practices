/*This code contains simple examples of loop structures used in Rust. 
To try any loop you want, uncomment the relevant function in the main function.
to run the code use \cargo run --bin loops_in_rust
*/

fn basic_loop_ver1(){
    //basic loop: loops until the break keyword
    let mut counter = 0;
    loop {
        println!("Hello World - {}", counter);

        if counter == 5 {
            break; // the break key word breaks the loop 
        }

        counter +=1;
    }
}

fn basic_loop_ver2(){
    
    let mut counter = 0;

    let result = loop { // loop can return the variable or value written after the break keyword.
        println!("Hello World - {}", counter);// it prints the line for every loop

        if counter == 5 {
            break counter; // the break key word breaks the loop and returns counter
        }

        counter +=1;
    };

    println!("Result is : {}", result); // it will print the result of loop returns
}

//it loops till the condition get False
fn while_loop(){
    let mut counter = 0;

    while counter != 5 {
        println!("Counter in loop : {}",counter); // last value will be 4 because when counter get 5 the condition will be false
        counter +=1;
    }

    println!("The Loop Completed!");
    println!("Counter after loop : {}",counter);
}

//it loops for every elements in an array
fn for_loop_ver1(){

    let mut my_array = [10,20,30,40,50]; // creating an array with 5 i32 elements

    for element in my_array.iter(){ //calling 'for' with an iterator of array
        println!("The value is : {}", element);

    }
}
// it loops through all values ​​in the defined range
fn for_loop_ver2(){

    for number in 1..4{ //including the first value, excluding the second
        println!("The number is : {}", number); //it loops for 1,2,3
    }

}


fn main(){

    //basic_loop_ver1();
    //basic_loop_ver2();    
    //while_loop();
    //for_loop_ver1();
    //for_loop_ver2();

}

