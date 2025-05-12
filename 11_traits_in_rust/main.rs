use std::fmt::{Debug, Display};


fn traits_with_structs(){

    //defining the trait
    pub trait Summary {
        fn summarize(&self) ->String; //defining just the body
    }


    pub struct News{
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    //implementing a trait (in this case Summary trait for News Struct)
    //we have to define the method in trait here, else we get an error
    impl Summary for News {
        fn summarize(&self) ->String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    pub struct Tweet{
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    //implementing a trait (in this case Summary trait for Tweet Struct)
    //we have to define the method in trait here, else we get an error
    impl Summary for Tweet {
        fn summarize(&self) ->String {
            format!("{}, by {}", self.username, self.content)
        }
    }

    //lets create structs and try to call methods
    let news =  News{
        author: String::from("Joe"),
        headline: String::from("Big News"),
        content: String::from("just a lie"),
    };

    let my_tweet = Tweet{
        username: String::from("Joe"),
        content: String::from("The moon is blowing"),
        reply: true,
        retweet: true,
    };

    println!("Tweet Summary : {}", my_tweet.summarize());
    println!("News Summary : {}", news.summarize());

}


fn default_trait_implementation(){
    
    //defining the trait with default implementation
    pub trait Summary {
        //defining the method eith body
        // if any struct use this trait and implement a summarize method, it override this default one,
        //but if a struct just define this trait just with body, it calls the default summarize method 
        fn summarize(&self) ->String{
            String::from("Read More")
        } 
    }
   
    pub struct News{
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    //implementing a trait (in this case Summary trait for News Struct)
    //we have to define the method in trait here, else we get an error
    impl Summary for News {

        //Overriding the default Summarize method
        fn summarize(&self) ->String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    pub struct Tweet{
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    //İmplementing the Summary method as default
    impl Summary for Tweet{}

    //lets create structs and try to call methods
    let news =  News{
        author: String::from("Joe"),
        headline: String::from("Big News"),
        content: String::from("just a lie"),
    };

    let my_tweet = Tweet{
        username: String::from("Joe"),
        content: String::from("The moon is blowing"),
        reply: true,
        retweet: true,
    };

    println!("Tweet Summary : {}", my_tweet.summarize());
    println!("News Summary : {}", news.summarize());


}

fn traits_as_parameters(){

    //this function take a reference of some thing which implement Summery traits and call its implemented method
    //simple syntax
    pub fn notify(item: &impl Summary){
        println!("Breaking news {}", item.summarize());
    }

    //long syntax
    //it means All types but just only implements Summery generic function with a argument that referance of this generic(all but implements Summary)
    pub fn notify_l<T: Summary>(item: &T){
        println!("Breaking news {}", item.summarize());    
    }

    //long sysntax with two arguments. both arguments must be the same Type and must implement Summary
    pub fn notify3<T: Summary>(item1: &T, item2:&T) { }

    //more than one implementation requirement
    pub fn notify4<T: Summary + Display>(item1 :&T, item2:&T){  }

    //if we need more types and more implementation requirement for each we can use "where clouse"
    pub fn notify5<T,U>(item1: &T, item2: &U)->i32 
        where T: Display + Clone,
              U: Clone + Debug
              { //Note that the semicolumn starts here 
                //...
                15
              }

    pub trait Summary {
        
        fn summarize(&self) ->String;
        
    }

    pub struct News{
        pub author: String,
        pub headline: String,
        pub content: String,
    }
    
    impl Summary for News {

        //Overriding the default Summarize method
        fn summarize(&self) ->String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    let news =  News{
        author: String::from("Joe"),
        headline: String::from("Big News"),
        content: String::from("just a lie"),
    };

    notify(&news);

}

fn traits_as_return_type(){

    pub struct Tweet{
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    pub trait Summ{
        fn summirize(&self) ->String;
    }

    impl Summ for Tweet{
        fn summirize(&self) -> String{
            format!("Read More from {}...", self.username)
        }
    };

    //it retruns a Tweet. Tweet implement Summ, so this function retruns an item which implement Summ
    fn return_summarizable() -> impl Summ{
        Tweet{
            username: String::from("Joe"),
            content: String::from("The moon is blowing"),
            reply: true,
            retweet: true,
            }
        }
    
    println!("{}", return_summarizable().summirize());

}

fn conditionally_implement_methods(){

    struct Pair<T> {
        x: T,
        y: T,
    }

    //a constructor method for Pair<T> this method can call by all types
    impl<T> Pair<T> {
        fn new(x:T, y:T) -> Self {
            Self{x,y}
        }
    }
    //This method can only be called by x and y that meet the conditions.
    impl<T: Display + PartialOrd> Pair<T>{
        fn cmp_display(&self){
            if self.x >= self.y {
                println!("The largest member is x");
            }else {
                println!("The largest member is x");
            }
        }
    }

}


fn main(){
    //traits_with_structs();    
    //default_trait_implementation();
    //traits_as_parameters();
    //traits_as_return_type();
    conditionally_implement_methods();
    


}