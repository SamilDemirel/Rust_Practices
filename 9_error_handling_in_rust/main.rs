use std::io;


fn panic_macro(value : u8){

    if value==0{
        panic!("Invalid Value")
    }else{
        println!("Done");
    }
    
}

fn result_enum(){

    // result enum is in the rust library, here is how it looks like
    /* enum Result<T,E>{
          Ok(T),
          Err(E),
        }
    */
    
    //Functions that are likely to cause panic usually return a result enum.
    use std::fs::File;
    // next line use fs::File to open a file. if the file does not exist return Err(error), if it does exist retruns Ok(file)
    let my_file = File::open("hello.txt");
   
    //now we must check the result with match
    let my_file = match my_file{
        Ok(file) => file,
        Err(error) => panic!("Error while oppening the file {:?}",error),
    }; 
}

fn handle_error(){
    //when run the code, Rust will panic and you will se this line at the backtrace
    //Error while oppening the file Os { code: 2, kind: NotFound, message: "........" }

    //if we catch the error and do something else without panic we use error kind
    use std::io::ErrorKind;
    use std::fs::File;

    let my_file = File::open("hello.txt");
    
    let my_file = match my_file{
        Ok(file) => file, //if succses return file 
        
        //if fail get the error and chek the kind of error by match
        Err(error) => match error.kind(){
                        //if the error kind is NotFound try to create new file( Creating file can also panic so we need to check its result enum to) 
                        ErrorKind::NotFound => match File::create("hello.txt"){
                                                     Ok(new_file) => new_file, //bound and return file
                                                     //if there is an other error then panic
                                                    Err(e) => panic!("Problem creating new file {:?}", e)},
                        _ => panic!("Problem opening teh file"),//for the other error but NotFound
                        },   
        };
    
}

fn unwrap_method_of_result(){
    use std::fs::File;
    
    //uncomment the next line for try unwrap method
    //let my_file = File::open("hello.txt").unwrap();//
    //if unwrap returns error it panics
    
    //if we want to add a panic message we can use expect method of Result enum
    let my_file2 = File::open("hello.txt").expect("No Such A File To Open");

   
    //we can use wrap method of Result enum. There is no need to use match. unwrap returns file or panic

    //let my_file = match my_file{
    //    Ok(file) => file,
    //    Err(error) => panic!("Error while oppening the file {:?}",error),
    //}; 

}

fn return_result_of() -> Result<String, io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut my_file = File::open("hello.txt")?; //when we use '?', if open succseed my_file get the file, if fail, my_ 

    let mut line = String::new();
    my_file.read_to_string(&mut line)?; //when we use '?', if reading succseed line will be a string else Error

    Ok(line)

}

//we use panic macro if the error can not be handeled in any way, and the program must stop
//we use Result Enum to handle error to prevent crashing
//we use except and unwrap when we prototyping, because they are simple and fast 
//After finishing prototyping we retruns all excepts and unwraps to Result Enum to prevent crashing


fn main(){
    //call the function by passing 0 and see the panic and back trace on the terminal
    //panic_macro(0);
    //result_enum();
    //handle_error();
    //unwrap_method_of_result();
    let result =  return_result_of();
    println!("{:?}", result);

}