/* we can use references by slicing Strings or arrays by their indexes */

fn slicing_string(s:&String){
    
    let first_word_ref = &s[0..5]; // it refers to indexes between 0 to 5(exclusive)
    let second_word_ref = &s[6..11]; //it refers to indexes between 6 to 11(exclusive)
    let all_string_ref =&s[..]; //it refers all String by all indexes (when we use it String returns to literal str)

    //we can simplfy the slices like
    // &s[..5] from the begining to 5
    // &s[6..] from 6 to the end

    println!("First Word : {}", first_word_ref);
    println!("Second Word : {}", second_word_ref);

    println!("All Words : {}", all_string_ref);
}

fn slicing_array(my_array :[i32;5]){

    let slice1 = &my_array[0..2];
    let slice2 = &my_array[3..5];

    for i in slice2.iter(){ // or you can try slice1.iter()
        println!("{}", i);
    }
}


fn main(){
    let my_string = String::from("Hello World");
    slicing_string(&my_string);
    println!("Original String : {}", my_string);

    let arr = [1,2,3,4,5];
    slicing_array(arr);
}