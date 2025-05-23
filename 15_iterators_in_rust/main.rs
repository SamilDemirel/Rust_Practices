

fn main(){

    let my_vec = vec![1,2,3,4,5];
    
    //creating an iterator over my_vec
    let my_iterator = my_vec.iter();

    //using the iterator to iter over vector's items
    for num in my_iterator{

        println!("Number is : {}", num);
    }

    //iterator_sum();
    //iterator_map();
    //iterator_filter();
    my_own_iterator();

}

fn iterator_map(){
     let v1 = vec![1,2,3];

    //map function gets a closure as parameter, 
    //next line use the iterator over v1 and runs the closure for each item then put the new items in a vector by collect() function
    let mut v2: Vec<i32> = v1.iter().map(|x|x+10).collect();

    for num in v2.iter(){

        println!("New Number is : {}", num);
    }
}

fn iterator_sum(){

    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();

    //sum() function can sum multiple types, so we need to define return type)
    let total: i32 = v1_iter.sum();

    println!("Total is : {}", total);

}

fn iterator_filter(){

    let full_vec = vec![1,2,3,4,5,6,7,8,9];

    //The filter function returns an iterator for values ​​that the statement returns true and adds the value to the new vector, 
    //and does not process values ​​that return false.
    //The filter function takes a reference parameter. The value itself must be dereferenced to be used in the statement.
    let greater_then_five: Vec<i32>= full_vec.into_iter().filter(|num| *num > 5).collect();

     for n in greater_then_five.iter(){

        println!("New Number is : {}", n);
    }
}

fn my_own_iterator(){

    struct Counter {
        count: u32,
    }
    
    impl Counter{
        fn new() -> Counter{
            Counter{count : 0}
        }
    }

    impl Iterator for Counter{
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {

            if self.count < 5{
                self.count +=1;
                Some(self.count)
            }else {
                None
            }
            
        }
    }

    //it should be mutable to change the value of counter
    let mut my_counter = Counter::new();

    loop {
        let value = my_counter.next();

        if let Some(val) = value{
            println!("Count is : {}", val)
        }else {//means None
            println!("NoneValue Returden, End Of Iteration");
            break;
        }
    }


}



//Iterators defined in standart library as trait like this

pub trait Iterator {
    type Item;
    // next needs mutable referance
    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterrator_demostration(){
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();
    /*if we need mutable referances of the items we can use:
         v1.iter_mut() 

      or if we need values it selfes we use:
        v1.into_iter()
        (retruned value will be Some(1), Some(2))
    */

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&6));
    assert_eq!(v1_iter.next(), Some(&3));
    //next() retruns None after the last item
    assert_eq!(v1_iter.next(), None);

}



