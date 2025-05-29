use std::ops::Deref;

fn simple_deref(){

    let x = 5; //x is a variable that hold the int value 
    
    let y = &x; // y is a reference and it olds the adress of the value of x

    assert_eq!(5,x); // 5==x
    assert_eq!(5,*y) // '*y' means "Get the value pointed to by the address in y"  so *y == 5

}

fn deref_with_smart_pointers(){

    let x = 5; //x is a variable that hold the int value 
    
    let y = Box::new(x); // y is a Box smart pointer which holds the adress
    

    assert_eq!(5,x); // 5==x
    assert_eq!(5,*y) //Box has a deref method so we can call it by using '*'

}

fn deref_with_own_smart_p(){

    //defining a struct with generic T type
    struct MyBox<T>(T);

    //implementing a constructor
    impl<T> MyBox<T> {

        fn new(x:T) -> MyBox<T>{
            MyBox(x)
        }
    }

    //implementing Dref trait and method for MyBox    
    impl<T> Deref for MyBox<T>{

        type Target = T; 

        fn deref(&self) -> &Self::Target {
            //&self is a tuple which holds adress, for derefing we need to return the adress which in index 0
            &self.0
        }

    }  

    let x = 5;
    
    let y = MyBox::new(x);
    
    assert_eq!(5,x); // 5==x
    assert_eq!(5,*y) // '*' calls the deref method of our MyBox.  '*' means *(y.deref())  

}

// When we call '*' Rust returns a reference to the value instead of the actual value. This is due to Ownership rules.


fn deref_with_string(){
    // The deref method is also defined for String.//
    fn hello(name: &str){
        println!("Hello, {}", name);
    }

    struct MyBox<T>(T);

    impl<T> MyBox<T> {

        fn new(x:T) -> MyBox<T>{
            MyBox(x)
        }
    }
    
    impl<T> Deref for MyBox<T>{

        type Target = T; 

        fn deref(&self) -> &Self::Target {
            &self.0
        }

    }  

    let new_mybox = MyBox::new(String::from("Jack"));
    hello(&new_mybox);
    //we pass a reference for new_mybox and its accualy this
        //&Mybox<String>
    //Mybox calls deref method and returns
        //&String
    //Rust calls deref method for String to and retrun a string slice 
        //&str
    
    // &Mybox<String> -> &String -> &str this chain runs automaticly on compile time
    //if we want to define it;
        //hello(&(*m)[..])

    /*Rust does deref coercion when it finds types and trait implementations in three cases;
        1.From &T to &U when t: Deref<Target = U>
        2.From &mut T to &mut U when T: DerefMut<Target=U>
        3.From &mut T to &U when T: Deref<Target=U>

        !! Rust does not deref corecion for
            -a &T to &mut U  
    */


}


fn drop_trait(){
    // drop methods calls by rust when a reference drops out of the scope and free the memory
    // we can define our own drop method
    // and we can drop a reference by calling drop(variable_name) function befor dropping out of scope manualy

    struct MySmartPointer{
        data : String,
    }

    impl Drop for MySmartPointer{
        fn drop(&mut self){
            println!("Dropping with data: {}", self.data);
        }
    }

    let c = MySmartPointer{
        data: String::from("my first pointer data"),
    };

    let d = MySmartPointer{
        data: String::from("my second pointer data"),
    };

    println!("Custom pointers created");
    //after creating custom pointers and printed to terminal, scope will end and
    //Firstly, Rust will call specific drop method for d,
    //secondly will call the method for c (simple LIFO)

    //when we use lock, we can need to drop variable befor getting out of scope
    //in this case we can call drop() function

    //if you uncomment next line, You will see that the dropping order has changed, 
    //first c is dropped (because of the drop function) and then d is dropped.    
    drop(c); //we cant call c.drop() directly. we have to use drop() function
    println!("Now going out of scope");

}

fn reference_counting(){

    // reference counter Allows different parts of the program to read the same data. Does not allow data to be changed 

    //creating a simple linked list
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    //bringing the enum to scope
    use List::{Cons,Nil};

    //creating a List with two item 
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //creating a List which point to list a
    // when we do that the ownership of the a pass to b, so we cant use a again.
    let b = Cons(3,Box::new(a));
    //if we create an other List which point to list a to, it gives an error 
    //because, the a is dropped 
    //let c = Cons(4,Box::new(a));  uncomment the line to see the error

    //if we try to use reference to use a in both list, we have to define lifetimes
    //But borrow checker does not allow it because it cant know the list's lifetimes

    //We need to define a new List with Rc<T> Reference Counting 

    use std::rc::Rc;

    enum MyList{
        MyCons(i32, Rc<MyList>),
        MyNil,
    }

    //creating a new list by wrapping Rc
    use MyList::{MyCons,MyNil};

    let x = Rc::new(MyCons(5, Rc::new(MyCons(10, Rc::new(MyNil)))));
    //we can use to see the value of the counter by Rc::stron_count(&z)
    //when a list drop out of scope, the counter descent automaticly
    println!("Count Of Reference after creating x: {}", Rc::strong_count(&x));
    
    //we have to call Rc::clone to pass a reference to rc
    {
        let y = MyCons(3,Rc::clone(&x));
        println!("Count Of Reference after creating y: {}", Rc::strong_count(&x));
    }
    
    let z = MyCons(4,Rc::clone(&x));
    println!("Count Of Reference after creating z: {}", Rc::strong_count(&x));
    // Rc::clone does not create a deep copy of list, it just clone the refrence of x list

}


fn interior_mutability(){

    //Borrowing rules in Box at compile time
    //But for RefCell, rules are applied at runtime
    //This is because it is impossible to detect certain features of a program using static analysis.
    //RefCell uses when you are sure that the code follows the borrowing rules but the compiler cannot understand it
    //RefCell cant be used in multithread programs

    use std::cell::RefCell;

    struct Counter {
    count: RefCell<u32>, // Internally mutable field
    }

    impl Counter {
        fn new() -> Self {
            Self {
                count: RefCell::new(0),
            }
        }

        fn increment(&self) {
            // We can mutate `count` even though `self` is immutable
            // we can change the value by calling borrow_mut() function
            *self.count.borrow_mut() += 1;
            

        }

        fn get(&self) -> u32 {
            // Borrow immutably to read the value
            //we can read the value by calling borrow() function
            *self.count.borrow()
        }
    }

    let counter = Counter::new();

    counter.increment();
    counter.increment();

    println!("Counter value: {}", counter.get());

    //If two different borrow_mut() calls are made at the same time, the program panics. (But this is not undefined behavior.)
    /*
        let x = RefCell::new(5);
        let a = x.borrow_mut();
        let b = x.borrow_mut(); // Panic! because `a` already mutably borrowed `x`

     */

    /*we can use with Rc to get multiple mutable reference for a variable
        enum List{
            Cons(Rc<RefCell<i32>>,Rc<List>),
            Nil,
        }
    */

    //When using RefCell, there is a small performance hit as borrowchecking will be done at runtime.

}

fn reference_cycle(){
    //Rust does not guarantee that there will be no memory leaks
    //Memory leaks may occur as a result of incorrect use of RC and Ref Cell.
    //Be careful with reference loops

    use std::cell::RefCell;
    use std::rc::{Rc,Weak};

    #[derive(Debug)]
    //creating a node for a tree structure
    //Nodes has a value and a Rc wrapped nodes vector which wrapped by refcell
    //we use Rc to reach the nodes from out side of the tree

    struct Node {
        value: i32,
        //we are defining a Weak smart pointer to keep the reference of the parent Node
        //we dont use Rc because it makes a reference circle and causes memory leak
        //Weak smart pointer is a small version of Rc that holds non-owning reference to managed allocation
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    //creating a node called leaf with value 3 and children as an empty vector
    let leaf = Rc::new(Node{
        value:3,
        //creating a new Weak smart pointer with empty value
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    //calling .borrow() to read the value and then calling upgrade() to return the Weak pointer to a Rc pointer 
    //Because a weak smart pointer does not know if the referenced value has dropped or not
    //it returns an Option Enum 
    println!("Parent of the leaf = {:?}", leaf.parent.borrow().upgrade());

    //creating another node called branch with value 5 and children with a Rc reference to leaf
    let branch = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    //calling borrow_mut() func of RefCell to assing a value to leaf node's parent
    //and assining the branch node's reference by calling Rc::downgrade() func
    //Rc::downgrade() retruns a Weak smart pointer with assigned reference
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    //after assigment 
    println!("Parent of the leaf = {:?}", leaf.parent.borrow().upgrade());

    //Rc smart pointers store 2 counts
        //1.strong_count: number of references that own the data
        //2.weak_count: number of references that does not own the data
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    //When the Rc<T> count goes to zero (i.e. all Rc pointers are dropped), the data in the heap is destroyed (dropped).
    //Any remaining Weak pointers will no longer point to the value, and calling .upgrade() on them will return None."



}

fn main(){
    //simple_deref();
    //deref_with_smart_pointers();
    //drop_trait();
    //reference_counting();
    //interior_mutability();
    reference_cycle();

}