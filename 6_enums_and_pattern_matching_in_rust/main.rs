

fn basic_enum(){
 
 enum IpAddrKind {
     V4,
     V6,
 }

 let four = IpAddrKind::V4;
 let six = IpAddrKind::V6;

}


fn enum_with_data(){

 #[derive(Debug)] // to print the enum   
 enum IpAddrKind { 

    V4_1(String), // now enum value can hold a data with String type
    V4_2(u8,u8,u8,u8), // this enum can hold four u8 data like an array
    V6(String),
 }

 let my_ip = IpAddrKind::V4_1(String::from("127.0.0.1"));
 println!("V4 with string ip{:?}", my_ip);

 let local_ip = IpAddrKind::V4_2(127, 0, 0, 1);
 println!("V4 with u8 ip{:?}", local_ip);

}

fn enum_with_method(){

    #[derive(Debug)]   
    enum IpAddrKind { 
        V4_1(String), 
        V4_2(u8,u8,u8,u8), 
        V6(String),
    }

    impl  IpAddrKind {
        // implementing an associated function
        fn print_enum(){
            println!("This is a associated function of IpAddrKind Enum");
        }

        // implementing a method 
        fn display(&self) -> String {

            //this match block returns a format depending on the self enum's type, it can print by println!
            match self {
                IpAddrKind::V4_1(s) => format!("IPv4 (string): {}", s),
                IpAddrKind::V4_2(a, b, c, d) => format!("IPv4: {}.{}.{}.{}", a, b, c, d),
                IpAddrKind::V6(s) => format!("IPv6: {}", s),
            }
        }
    }

    IpAddrKind::print_enum();

    let my_ip = IpAddrKind::V4_1(String::from("127.1.1.1"));
    println!("{}",my_ip.display());

}


fn option_enum(){

    /*this enum is already defined in the rust library. 
    
    enum Option<T>{
        Some(T),
        None,
    }
    
    */

    let x: i8 = 5;
    let mut y: Option<i8> = Some(5);

    //if we try to add these values at next line, it gives an error because we can't add different types  
    //let sum = x + y;
    // so we must unwrap the y value

    let sum = x + y.unwrap_or(0); 
    // if y has a value y.unwrap_or returns the value, but if y does not have a value (it means its None!) returns the default argument value '0'
    println!("Sum is : {}", sum);// it prints 10

    let y: Option<i8> = None;
    let sum = x + y.unwrap_or(0); 
    println!("Sum with new y value is : {}", sum); // it prints 5

}

fn match_expression(){

    enum Coin {
        PENNY,
        NICKEL,
        DIME,
        QUARTER,
    }

    fn value_in_cent(coin:Coin) -> u8{
        
        //match expression must inclue all options of the enum. othervise it gives an error
        //The code corresponding to the value of the Coin entered into the function as a parameter is run.
        //If only the value is returned in a single line, => is sufficient. For more than 1 line we must use {}
        match coin{
            Coin::PENNY => 1,
            Coin::NICKEL => 5,
            Coin::DIME => 10,
            Coin::QUARTER => {
                println!("This is a Quarter");
                25
            }
        }
    }

    let my_coin = Coin::QUARTER;
    let cents = value_in_cent(my_coin);
    println!("The cent is :{}", cents);

}

fn combine_enum_values(){

    #[derive(Debug)] 
    enum States {
        Alabama,
        Alaska,
        Arizona,
    }

    enum Coin {
        PENNY,
        NICKEL,
        DIME,
        QUARTER(States),
    }

    let my_coin = Coin::QUARTER(States::Alabama);

    let cents =  match my_coin{
                        Coin::PENNY => 1,
                        Coin::NICKEL => 5,
                        Coin::DIME => 10,
                        Coin::QUARTER(state) => {// we are bounding the State enum value of my_coin to state variable
                            println!("The State value Is : {:?}", state); //and printing the state value
                            25
                        }
                    };
    println!("The cent is :{}", cents);
}

fn a_trick_for_match(){

    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x {
             // bounding variable i to x value in Some()
            Some(i ) => Some(i+1), // if x is not None, return Some(x+1) 
            None => x,// else return Some(x) back
        };

        // if there are lots of enum options its hard to write all lines for all options
        // we can write the lines just we want to use in match and add one more line with _ for the rest
        match x {
            Some(i) => Some(i+1),
            _ => x, //for the rest of the options returns Some(x) i and x are bounded!
        }

    }

    let result = plus_one(None);
    println!("The Result Is: {}", result.unwrap_or(0));

}

fn if_let_syntax(){

    let my_value = Some(5);

    //we must specify all options or one we need and _ for the rest   
    match my_value {
        Some(5) => println!("The Value is five"),
        _ => println!(""),
    }
    
    //we can specify only the option we need by using if-let syntax.
    // if my_value is Some(5) then println! else do nothing 
    if let Some(5) = my_value{
        println!("The Value is five");
    }

}


fn main(){
    //basic_enum();
    //enum_with_data();
    //enum_with_method();
    //option_enum();
    //match_expression();
    //combine_enum_values();
    //a_trick_for_match();
    if_let_syntax();
        
}