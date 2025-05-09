
// These imports allow us to get the type name and memory size of a variable.
// We'll use them to inspect data types, and we'll cover their details later.
use std::any::type_name;
use std::mem::size_of;

//---------------------------SCALER DATA TYPES-------------

//call this function to see the types and memory sizes of each signed integer type
fn signed_integer_data_types(){
    
    //Signed Integers. The value can be positive or negative (+/-) 
    let a: i8 = 4;
    let b: i16 = 4;
    let c: i32 = 4; // this is the default integer type of Rust. 
    let d: i64 = 4;
    let e: i128 = 4;
    let f: isize = 4; // varies by architecture

    //If you assign an integer to a variable without specifying the type, Rust will default to i32.
    let g = 4;
    
    print_type_and_size(&a,"a");
    print_type_and_size(&b,"b");
    print_type_and_size(&c,"c");
    print_type_and_size(&d,"d");
    print_type_and_size(&e,"e");
    print_type_and_size(&f,"f");
}
    
//call this function to see the types and memory sizes of each signed integer type
fn unsigned_integer_data_types(){

    //Unsigned Integers. value can only be positive integer 
    
    let g: u8 = 4;
    let h: u16 = 4;
    let i: u32 = 4;
    let j: u64 = 4;
    let k: u128 = 4;
    let l: usize = 4; // varies by architecture
       
    print_type_and_size(&g,"g");
    print_type_and_size(&h,"h");
    print_type_and_size(&i,"i");
    print_type_and_size(&j,"j");
    print_type_and_size(&k,"k");
    print_type_and_size(&l,"l");
       
}

fn different_representing_of_integers(){

    let a: i32 = 98_222;      //Decimal
    let b: i32 = 0xFF;        //Hex
    let c: i32 = 0o77;        //Octal
    let d: i32 = 0b1111_0000; //Binary
    let e: u8  = b'A';        //Byte   

    print_type_and_size(&a,"a");
    print_type_and_size(&b,"b");
    print_type_and_size(&c,"c");
    print_type_and_size(&d,"d");
    print_type_and_size(&e,"e");
}

fn integer_overflow(){
    //Each data type has a maximum value that it can take.
    // u8 values can between 0 - 255
    // i8 values can between -128 - 127
    // u32 values can between 0 - 4_294_967_295 etc..

    let x: u8 = 255;
    println!("Value of x: {}", x);
    // when we try to assign 256 to an u8 variable and run the code, Rust panics "error: this arithmetic operation will overflow"
    //let y: u8 = x +1;
    //println!("Value of y: {}", y);
    
    //If you try to assign a value that exceeds the maximum limit of the type, you will get an error.
    //uncomment the next two lines and see the error
    //let z: u8 = 256;
    //println!("Value of z: {}", z);
}


fn floating_point_number(){

    let a: f32 = 4.0;
    let b: f64 = 4.0; //f64 is the default 

    print_type_and_size(&a,"a");
    print_type_and_size(&b,"b");
}

fn boolean_type(){
    //8 bits in memory
    let a: bool = true;
    let b =false;

    print_type_and_size(&a,"a");
    print_type_and_size(&b,"b");

}

fn character_type(){
    //32 bits in memory
    let a: char = 'a';
    let b = 'b';

    print_type_and_size(&a,"a");
    print_type_and_size(&b,"b");

}


//---------------------------COMPOUND DATA TYPES-------------

fn tuple_data_type(){

    let my_tuple = ("Hello World", 15); // scaler data types automatically defined

    //destructure of tuple
    let (text, number) = my_tuple; // scaler data types automatically defined
    
    println!("Destucturel Value of text: {}", text);
    println!("Destucturel Value of number: {}", number);
    
    //dot notation
    let text_dot = my_tuple.0;  //using (tuple_name.indexnumber) for dot notation
    let number_dot = my_tuple.1;

    println!("Dot Notation Value of text: {}", text_dot);
    println!("Dot Notation Value of number: {}", number_dot);
}

fn array_data_type(){

    // Rust arrays are fized size
    let my_array1 = [10, 20, 30, 40]; //automatically defined data type(i32) ; fixed array size(4)
    println!("First item in array: {}", my_array1[0]);
    println!("Second item in array: {}", my_array1[1]);

    //Define an array with initial values ​​of 0 and length of 8.
    let my_array2 = [0; 8];

    //printing all the items in array
    print!("Items of my_array2 : [");
    for item in my_array2.iter(){
        print!("{}",item);
        print!(", ");
    } 
    println!("]");
}

   
fn main(){

    //signed_integer_data_types();
    //unsigned_integer_data_types();
    //different_representing_of_integers();
    //integer_overflow();
    //floating_point_number();
    //boolean_type();
    //character_type();
    //tuple_data_type();
    array_data_type();

}




/*This function prints the type and memory size (in bits) of a given variable.
Our main goal here is to understand data types and how much space they occupy in memory.*/
fn print_type_and_size<T>(_: &T, name: &str) {
    println!("Variable Name: {}", name);
    println!("Type of the variable: {}", type_name::<T>());
    println!("Size of the variable: {} bits", size_of::<T>() * 8);
    println!("---------")
}
