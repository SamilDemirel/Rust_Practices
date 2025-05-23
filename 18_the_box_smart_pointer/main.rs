
//We use Box smart pointer when:
    // when we have a type whose exact size is not known at compile time
    //Large data structures, (to move to heap to prevent copying)
    //Recursive data structures, (for compiler type understanding)
    //Trait objects (dyn Trait), (for dynamic dispatch)
    //For performance optimization, (for space holding)



fn main(){

    let b = Box::new(5);
    //5 value stored on heap via a Box smart pointer
    //b value is a pointer(keeps the adress of the Box) on stack which points to box that we created on heap

    println!("b= {}", b); //this line print the value not the adress

    //when Box dropped from scope, the b also drop and both memories (heap and stack) dealocated automaticly
}

fn recursive_data_structers(){

    /*
    //uncomment the enum and see the error
    enum MyList {
       Cons(i32, My_list), //compiler can not know how much this recursion repeats 
       Nil,
    }
    
    */

    enum MyList {
        Cons(i32, Box<MyList>), //now compiler knows how much memory need on stack (just an adress will store to stack)
        Nil,
    }

    use MyList::{Cons, Nil};

    //creating a new insatance of Mylist enum
    let list1 = Cons(1,Box::new(Cons(2,Box::new(Nil))));



}