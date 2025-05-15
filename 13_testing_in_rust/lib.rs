

#[cfg(test)]
    mod my_tests{
        use std::assert_eq;

        #[test]
        fn it_works(){
            assert_eq!(2+2,4);
        }

        //when a function or method panics the test fails
        #[test]
        fn faling_test(){
            panic!("Making this test fail manualy");
        }
        //to use the struct which we want to test, we need to get it to scope by use
        use super::*;
        #[test]
        fn test_if_larger_can_hold_smaller(){
            //creating 2 rectangle 
            let larger = Rectangle{
                width:8,
                height:7,
            };
            let smaller =Rectangle{
                width: 5,
                height: 1,
            };

            //we want to test, when i call can_hold method over larger and pass smaller (it have to retrun true) 
            // and when i call can_hold method over smaller and pass larger (it have to retrun false)

            // we can use assert! macro to check the returned boolean value is true
            assert!(larger.can_hold(&smaller)); 
            //next assert will true if method retrun false because of '!' (not char)    
            assert!(!smaller.can_hold(&larger));
        }

        #[test]
        fn test_if_add_to(){
            //we will test if add_two function is working fine
            // assert_eq macro retruns true if two parameters are equal, if not it panics and test fails
            //add_two(2) have to return 4 so, 4 must be equal 4
            assert_eq!(4, add_two(2));
        }

        #[test]
        fn test_if_add_to_by_assert_ne(){
            //we will test if add_two function is working fine
            // assert_ne macro returns true if two parameters are NOT EQUAL, if they are equal it panics and test fails
            
            assert_ne!(8, add_two(2));
        }

        #[test]
        fn test_if_greating_contains_name(){
            let result = greeting("Carol");
            //in this case, if first argument is false, assert macro panics and print the message that we write 
            assert!(result.contains("Carol"),
                    "Greeting did not contain name, value was {}", result); 
        }

        #[test]
        #[should_panic]// we define the passing condition as "function must panic" if does not the test faild
        fn greater_than_100(){
            Guess::new(200); 
        }
        
        //[should_panic] key word passes for every kind of panic
        // if we want to define that test should pass for only one spesific panic, we can use expected="defined_panic_message_in_function"
        
        #[test]
        //if the function panic and retruns a different message, this test fails
        #[should_panic(expected="Value must be between 1-100")]
        fn smaller_than_1(){
            Guess::new(-1); 
        }

        //testing with Result Enum
        //this test passes if the function return Ok(()), if it return Err(String) fails
        #[test]
        fn if_return_type_correct() -> Result<(), String> {

            if 2+3 == 4{
                Ok(())
            }else {
                Err(String::from("2 +2 does not equal 4"))
            }
        }

        //if a test takes tomuch time or resources, and if we dont want to run it for every time, we can ignore it
        #[test]
        #[ignore]
        fn expensive_test(){
            //code
        }
        //if we run just the ignored test via terminal
            //cargo test -- --ignored
        



}


#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

//------------------

pub fn add_two(a:i32) ->i32{
    a+2
}

pub fn greeting(name:&str) -> String{
    format!("Hello {}! ", name)
}

//------------------

pub struct Guess {
    value :i32,
}

impl Guess{

    pub fn new(value:i32) -> Guess {
        if value <1 || value > 100 {
            panic!("Value must be between 1-100");
        }

        Guess{value}
    }
}


//Command Line Options for Test

//as default, /cargo test runs the tests as different threads
//if we want to run test as paralel (one by one) we can use this prompt (usefull when different test modifying same file)
    //user& cargo test -- --test-threads = 1 (defining the threads number)

//If a test success, Rust does not show the print statement. 
//If we want to see the print statement of succseded test, we can use the following command line
    //user& cargo test -- --show-output

//if there are multiple tests and we only want to run one specific one
    //user& cargo test test_name (the function name of the test)
    // or user& cargo test the_initial_part_of_the_test_name 
    // or user& cargo test my_tests(the test module name)::test_name

// We have Two Main Test Catagories

    //1.Unit Tests (small and focused tests on methods, modules or functions, private interfaes)
    //2.Integration Tests (external libraries, public interfaces )

    //Integration Test:
        //we define a folder name tests for the integration tests
        //Cargo knows by default that it will find the integration tests in the tests folder 
                
        //Continue the integration tests topic from the integration_test.rs file in the test folder.
