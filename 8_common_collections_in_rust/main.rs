

//collections in rust are not-fixed sized so they are allocated on the HEAP

//1. VECTORS:
fn vectors_in_rust(){
    
    let mut my_vector :Vec<i32> = Vec::new(); //we are creating a vector instance with type of i32 by calling new() function
    //adding value in vector

    my_vector.push(1);
    my_vector.push(2);
    my_vector.push(3);
    
    println!("{:?}", my_vector);

    //we can create a vector by using vec! makro with initial values
    let mut my_vector2 = vec![1,2,3,4,5];
    println!("{:?}", my_vector2);

    //accessing the element of a vector
    //1. by index (not safe)
    let item3 = &my_vector2[2];
    println!("The third item in my_vector2 is {}", item3);

    //ATTENTION!!
    // if we use indexes to access it can cause a RUNTIME ERROR, because vectors sizes are inmutable and cant be known at compile time.

    //2. by get() method
    match my_vector2.get(12){// get() methods returns an Option Enum
        Some(third) => println!("The third item in my_vector2 is {}", third), //binding third to returned i32 value
        None => println!("no element on index"), //if index out of bound
    }

    //ATTENTION!!
    // by get() method or index we get the  reference of the item in vector
    // so if the vector mutable and if we still have the reference of any item in scope, 
    // we cant change the vector by push() or some other methods

    //iterating over elements
    for i in &my_vector2{ //using inmutable reference
        println!("{}", i );
    }
    //or
    for i in &mut my_vector2{ //using mutable reference so we can modify the vector
       //modifying the element by using dereference value '*'
       *i += 10;
    }

    for i in my_vector2.iter(){
       println!("After Modifying {}", i );
    }

    //Vectors can include only one type of data
    //if we need to use multiple type of data with vectors, we can use Enums
    
    #[derive(Debug)] 
    enum User{
        Name(String),
        Age(i16),
        Email(String),
    }

    let users_vector = vec![
                                            User::Name(String::from("armut")),
                                            User::Age(35),
                                            User::Email(String::from("hebele@gmail.com")),

    ];

    let mut idx = 0;
    for i in users_vector.iter(){
       println!("Enum Vector Element {} is : {:?}", idx, i);
       idx +=1;
    }

    //or we can declare a condition for searching a spesific data
    for item in &users_vector{
        match item{
            User::Age(i) => println!("{}",i),
            _ =>println!("Not An Age Value"),
        };
    }
}
    
fn strings_in_rust(){
        //in Rust, Strings are stored as a collection of UTF-8 encoded bytes

        let s1 = String::new(); //creating an empty String instance
        let s2 ="Hello World"; //this is a fixed size literaly string not a String instance
        let s3 = s2.to_string(); //creating a String instance with the value of s2
        let s4 =String::from("Hello World"); //almmost same as previous line 

        let strings = [s1,s3,s4];

        for s in strings.iter(){
            println!("{}", s);
            //for s1, nothing will printed because its an empty instance
        }

       //------------------
       let mut my_string = String::from("Hello");

       my_string.push(','); //it pushes a single character at the end of the String
       my_string.push(' ');

       my_string.push_str("World"); //it appends a String at the end of the String 
       
       println!("After manipulation my_string is {}", my_string);

       //we can also append Strings by using (+)
       let s1 = String::from("hello");
       let s2 = String::from(" world");
       let s3 = s1 + &s2; // we can use (+) with only an ownership and a reference
       
       println!("After (+) s3 is {}", s3); 

       //we can also append Strings by using format macro.
       let s4 = String::from("red");
       let s5 = String::from(" cat");
       let s6 = format!("{}{}", s4, s5); //the format macro does not take ownerships so we can use variables

       println!("After format! s6 is {}", s6); 

       /*
       Strings Unicode da 3 şekilde temsil edilir
       1. Bytes
       2. Scaler Value (fundamental Unicode character code points)
       3. Grapheme (normal visuliated character)
       */

      let hello = String::from("Hello");
      //let c:char = hello[0];  !! We cant use indexes with Strings in Rust

      //we can use into_bytes(returns a vector) or hello.bytes(returns a iterator)  
      let bytes_of =hello.into_bytes(); //it also borrow the ownership of hello value, and hello drops
      for b in bytes_of.iter(){
        print!("{},", b );
      }
       println!(" ");

     // we can use .chars(returns iterator) to print Scale Values!!!
     // Remember, in some different languages, scale values can be differnt than graphemes
     let my_word = String::from("Hello");  
     for c in my_word.chars(){
        print!("{},", c );
     }
     println!(" ");

    //For to be able to print graphemes for all languages, we need a import a dependencie
    // unicode-segmentation = "1.12.0"
    use unicode_segmentation::UnicodeSegmentation; //we will use it for indexing a String
    let my_word2 = String::from("World");  
    for g in my_word2.graphemes(true){
        println!("{}",g);
    }

}

fn hash_maps_in_rust(){
    //we must get the HashMap from standart lib to our scope
    use std::collections::HashMap;

    //HashMaps stores the data pair as key,value

    let blue_team = String::from("Blue");
    let red_team = String::from("Red");

    let mut scores = HashMap::new(); //creating an empty HashMap instance

    //if we pass Strings, Strings's ownership pass to hashmap!!
    //if we pass by reference we must use life times!!
    scores.insert(blue_team,10); //first value is the KEY, and the second value is VALUE 
    scores.insert(red_team,20); 

    for s in scores.iter(){
        println!("{:?}", s)
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //it retruns a Option<&i32> to prevent invalid key:value 
    println!("Team:{} Score: {:?}", team_name, score);

    scores.insert(String::from("Blue"), 20); // it override the old value 10, makes the pair ("blue",20)
    
    //if there is not the key, insert it with the default value
    //if there is an entry with the key word, do nothing
    scores.entry(String::from("Green")).or_insert((30)); // this line inserts the map, because the value doesnt exist
    scores.entry(String::from("Green")).or_insert((40)); // this line does nothing, but returns the reference of the exist key's value 
                                                                      
    for s in scores.iter(){
        println!("After using entry function{:?}", s)
    }

    //here is a word counter with entry function
    let text = " hello world hello again";
    let mut my_map = HashMap::new();

    for word in text.split_whitespace(){ //it retruns [hello,world,hello,again]
        let count = my_map.entry(word).or_insert(0); //or_insert return a mut reference to the value passed as default
        *count +=1; 
        //dereferencing the value which or_insert returns and add 1. When we add 1 to dereferenced value,
        //it changes the value in memory. Because it is a reference. 
        //if the key is exist in the map, .entry does not insert but or_insert retruns the reference of the value, and we add 1 it.
     }

    println!("{:?}",my_map);
}



fn main(){

    //vectors_in_rust();
    //strings_in_rust();
    hash_maps_in_rust();


}