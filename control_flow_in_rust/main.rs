//regular if_else statement
fn if_else_exp(num1: i16, num2:i16) -> i8{
    
    if num1 > num2 {
        1 //return 1
    } else if num1 == num2 {
        0
    } else{ // if num1< num2
        -1 
    }
    
}

//we can use if_else block as an expression so it returns value depends on condition 
fn if_else_with_let(num1:i8, num2:i8) ->i8{

    //if the condition is true return 1, if false return  0 
    if num1==num2 {1} else {0}
}



fn main(){

    let result = if_else_exp(8,7);
    println!("Result is : {}", result);

    let result2 = if_else_with_let(5,4);
    println!("Result2 is : {}", result2);

}