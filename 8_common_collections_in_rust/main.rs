//collections in rust are not-fixed sized so they are allocated on the HEAP

//1. VECTORS:
fn vectors(){
    
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

fn main(){

    vectors();


}