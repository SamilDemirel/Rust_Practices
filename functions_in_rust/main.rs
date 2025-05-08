//Defining a function that takes no parameters and no return value
fn function_no_params(){
    println!("Hello World");
}

//Define a function that takes one parameter and no return value
fn function_one_param(number: i32){
    println!("Param value is : {}", number);
}

//Define a function that takes two or more parameters and no return value
fn function_two_param(num1: i32, num2:i32){

    let total = num1 + num2;
    println!("Total is : {}", total);
}

//Define a function that takes two or more parameters and RETURNS VALUE
// use return key word and add an arrow with return type at the defination of function
fn sum1(num1: i32, num2:i32) -> i32{

    let total = num1 + num2;
    return total;
}

//or we can simplfy return statement
fn sum2(num1: i32, num2:i32) -> i32{

    let total = num1 + num2;
    total //Be careful! If the return keyword is not used, there should be no semicolon.
}

//or we can more simplfy return statement
fn sum3(num1: i32, num2:i32) -> i32{

    num1+num2 //Be careful! If the return keyword is not used, there should be no semicolon.
}

fn main(){

    //function call
    function_no_params();

    //function call with one param
    function_one_param(100);

    //function call with one param
    function_two_param(10,20);

    //Calling a function that returns a value
    //Defining a new variable for the value returned by the function.
    let total = sum3(10,20);
    println!("Total = {}", total);


}