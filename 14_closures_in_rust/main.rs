use std::thread;
use std::time::Duration;
//Closures capture all variables defined in the scope, if a variable is defined in the scope, it can be accessed directly from the closure.
//When defining closures, the type is not specified. 
//The type that the closure is first called with and the type that it returns first becomes the type.
/*Closures implement one of the following traits:

    Fn: reads only environmental variables.
    FnMut: changes environmental variables.
    FnOnce: owns (moves) environmental variables.

    // we can force closure to take ownership of variables by "move" keyword
    let my_closure = |z| z==x; this closure does not take the ownership of x
    let my_closure = move |z| z==x; this closure does not take the ownership of x


*/

//Closures are often passed as arguments to higher order functions (map, filter, for_each).


fn simulated_exp_calc(intensity : u32) -> u32 {
    println!("Calculating Slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main(){

    let sim_intensity=10;
    let sim_random_number = 7;
   
    generate_workout(sim_intensity, sim_random_number);

}

/*Since the input types of closures depend on the values ​​passed when they are first called, 
we define a struct to enable them to be called with different types and to store the data returned after they are called. */
struct Cacher<T>    
    where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>, //the value is Option because its None for initialize
    }

impl<T> Cacher<T>
    where T: Fn(u32) -> u32,
    {
        //constructer
        fn new(calculation:T) -> Cacher<T>{
            Cacher { calculation, value: None }
        }
    
        fn value(&mut self, arg:u32) -> u32{
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }    

fn generate_workout(intensity: u32, random_number:u32){
    
    //We define a closure that has the same logic as the calculation function. 
    //And creating a Cacherstruct with the value which closure returns
    let mut simulated_exp_calc_closure = Cacher::new( |num| {
        println!("Calculating Slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        //we can call a fonnction to generate a workout
        //println!("Today, do {} pushups!", simulated_exp_calc(intensity));
        //println!("Today, do {} situps!", simulated_exp_calc(intensity));

        //or we can call Cache structs's value
        println!("Today, do {} pushups!", simulated_exp_calc_closure.value(intensity));
        println!("Today, do {} situps!", simulated_exp_calc_closure.value(intensity));

    } else {

        if random_number == 3 {
            println!("Take a breath!");
        }else {
            println!("Today, run for {} minutes", simulated_exp_calc_closure.value(intensity));
        }

    }
}