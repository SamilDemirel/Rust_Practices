//In Rust, we define shared behavior by traits(like inheritance in other languages)
/*  In order for a trait to be used as a trait object in Rust (i.e. in the form of a dyn Trait), it must be object safe.

    --Object Safety--
    All methods must have these properties
        a. Return type should not be self
        b. methods don not use generic types

    If we only going to use trait as a generic bound (like where T: MyTrait), then object safety is not required.
    
*/

pub trait Draw {
    fn draw(&self){

    }
}

pub struct Screen{
    //defining a vector which stores the references with Box smart pointer 
    //and the Box smart pointer holds only the elements which implement Draw trait (dyn Draw)

    pub components: Vec<Box<dyn Draw>>, //dyn keyword means Dynamic Dispatch

        /*
            Static Dispatch: This is when the compiler knows at compile time the concrete functions it will use for generic types.
                             Object safety is not required

            Dynamic Dispatch: This is when the compiler does not know at compile time what concrete functions it will use for generic types.
                              concrete functions are specified at runtime.
                              Object safety is required     
        */

    //we cant use Generic type like Vec<T> because,
        //Generic Types limited only one type. if we use the generic with an int first, all the other components must be int
}

impl Screen{
    pub fn run(&self){
        //itereting over components vector and drawing all components to screen via draw method
        for component in self.components.iter(){
            component.draw();
        }
    }
}


fn main(){
    //creating a new component by using trait object
    struct SelectBox{
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox{
        fn draw(&self) {
            //draw select box
        }
    }

    //creating a new instance of Screen struct
    let my_screen = Screen{
        //all components shoul be inside Box smart pointer
        components: vec![
            Box::new(SelectBox{
                width: 100,
                height: 100,
                options: vec![String::from("1"),
                              String::from("2"),
                              String::from("1")],
            }),

        ]
    };

    //drawing all components to screen 
    my_screen.run();


}

