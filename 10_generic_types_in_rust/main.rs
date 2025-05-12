


fn witohut_generic_types(){
    //Even if the operations performed by the functions are the same,
    //we normally need to define separate functions for each type of arguments for different types.

    //same functionalty for Vec<i32>
    fn get_largest_number(number_list: Vec<i32>)-> i32{
        let mut largest_num = number_list[0];

        for num in number_list{
            if num > largest_num {
                largest_num = num;
            }
        }
        largest_num
    }

    //same functionalty for Vec<char>
    fn get_largest_char(char_list: Vec<char>) -> char{
        let mut largest_char = char_list[0];

        for c in char_list{
            if c > largest_char {
                largest_char = c;
            }
        }
        largest_char
    }

    let my_num_list = vec![21,3,11,99,65];
    let largest_num = get_largest_number(my_num_list);
    println!("The largest number is: {:?}", largest_num);


    let my_char_list = vec!['a','b','c','d','e','f'];
    let bigest_char = get_largest_char(my_char_list);
    println!("The bigest char is: {:?}", bigest_char);

}


fn with_generic_type(){

    // using PatalOrd and Copy Trairs tolimit the type options
    fn get_largest <T: PartialOrd + Copy> (list: Vec<T>) -> T{ // defining the function with generic type argument and generic return type
       
        let mut largest = list[0];

        for item in list{
            if item > largest { //we cant use <, >, +, -, * ,/ vs. operants with every type. We have to limit the type options 
                largest = item;
            }
        }
       largest
    }

    //now we can use same get_largest function for Vec<i32> and Vec<char> both
    let my_num_list = vec![21,3,11,99,65];

    let largest_num = get_largest(my_num_list);
    println!("The largest number is: {:?}", largest_num);

    let my_char_list = vec!['a','b','c','d','e','f'];

    let bigest_char = get_largest(my_char_list);
    println!("The bigest char is: {:?}", bigest_char);

}

fn generics_with_structs(){

    struct Point<T>{
        x: T,
        y: T,
    }

    //we can create Point struct with i32 numbers and also with float numbers

    let point1 = Point{x:10, y:10};
    let point2 = Point{x:1.34, y:2.33};

    //but we cant create Point with two different type for each item
    // let point3= Point{x:10, y:1.43}; this line gives an error
    
    //If we want to use different types for the same Point

    struct Point2<T,U> {
        x:T,
        y:U,
    }
    //We can use all options for two different types.
    let point3 = Point2 {x:10, y:1.43};
    let point4 = Point2 {x:1.23, y:43};
    let point5 = Point2 {x:1.23, y:1.21};
    let point6 = Point2 {x:23, y:21};
}

fn generics_with_structs_impl(){

    struct Point<T>{
        x: T,
        y: T,
    }
    //we can use this impl function for every type of x
    impl<T> Point<T> {

        fn get_x(&self)-> &T{
            &self.x
        }
    }

    //we can use this impl function for only y values which types are f64
    impl Point<f64>{
        
        fn get_y(&self)-> f64{
            self.y
        }
    }

    let point1 =Point{ x:10, y:10};
    let x = point1.get_x();
    println!("x point : {}",x);

    //let y = point1.get_y(); //we get an error, we cant reach the get_y func because y's type is i32
    
    let point2 =Point{ x:10.1, y:10.5};
    let y = point2.get_y(); //its okay for f64 y
    println!("y point : {}",y);

    struct Point2<T,U>{
        x: T,
        y: U, 
    }

    impl<T,U> Point2<T,U>{

        fn mixup<V,W>(self, other_point: Point2<V,W>) -> Point2<T,W>{
            Point2{
                x: self.x,
                y: other_point.y,
            }
            
        }
    }

    let p5 = Point2{x:3, y:5};
    let p6 =Point2{x:'a',y:'b'};

    let p3 = p5.mixup(p6);
    println!("The mixed point is x:{}, y:{}",p3.x,p3.y);


}



fn main(){

    //witohut_generic_types();
    //with_generic_type();
    //generics_with_structs();
    generics_with_structs_impl();

}