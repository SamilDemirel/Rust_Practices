
//defining a struct

#[derive(Debug)] // we need this implementation for printing struct 
struct User {
    name : String,
    age : u16,
    email: String,
    active : bool,
}
//we can also define method for struct by implementation

impl User {
    fn calc_year_of_birth(&self) -> u16 { //method's first argument always be the self (mostly as a reference)
        
        2025-self.age  
    }
    //It is a function associated with the User structure, independent of the created user information.
    fn create_with_name(name:String) -> User { // this is a associated function. not a methos so it does not takes self.
        User {
            name: name,
            age:0,
            email:String::from("none"),
            active :true,
        }
    }
}


fn build_user (name :String, email: String, age:u16, active : bool) -> User {

    User {
        email : email,
        name : name,
        age : age,
        active : active,

        /*struct field names and params names are same so no need to write field name.
        User {
            email,
            name,
            age,
            active,
        }
        */
    }
}

fn main(){
    //creating a User
    //if we define the user1 as mut, we can change the values of struct
    let mut user1 = User{ 
        name : String::from("Samil"),
        email : String::from("hebele@gmail.com"),
        age : 42,
        active : true,
    };

    // or we can create user by the build function
    let user2 = build_user(String::from("Jack"),String::from("jack@gmail.com"), 45, true);

    let name = user1.name;
    let age = user1.age;

    println!("First User's Name is : {}", name);
    println!("First User's Age is : {}", age);

    println!("Second User's Name Is: {}", user2.name);

    //lets chance some values of user1
    user1.name =  String::from("Hebele");
    println!("First User's New Name is : {}", user1.name);

    //creating new user with some of other user's data

    let user3 = User {
        name : String::from("Jill"),
        email : String::from("jill@gmail.com"),
        ..user1 // for other fields, Rust takes thesame values of user1
    };

    println!("Third User's Name is : {}", user3.name);
    println!("Third User's Age is : {}", user3.age);// this value is same with user1

    //if we want to println struct with all fields
    println!("User1 is : {:#?}", user1);

    //lets use the method of struct
    let year_of_birth = user1.calc_year_of_birth();
    println!("User1 Born in : {}", year_of_birth);

    //lets use the associated function of struct

    let user4 = User::create_with_name(String::from("Armut"));
    println!("Fourth User's Name is : {}", user4.name);
    println!("Fourth User's Age is : {}", user4.age);



}